extern crate crgp;
extern crate fine_grained;
extern crate getopts;
extern crate serde_json;
extern crate timely;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

use fine_grained::Stopwatch;
use getopts::{Matches, Options};
use timely::dataflow::*;
use timely::dataflow::operators::*;

use crgp::social_graph::edge::*;
use crgp::timely_extensions::Sync;
use crgp::timely_extensions::operators::Reconstruct;
use crgp::twitter::*;

/// The exit codes returned by the program.
pub enum ExitCode {
    /// Successful (i.e. expected) execution (Code: `0`).
    Success = 0,

    /// Invalid program parameters (Code: `1`).
    IncorrectUsage = 1
}

/// Execute the program.
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let program: &str = &arguments[0].clone();

    // Define defaults.
    let default_batch_size: usize = 500;

    // Define the program options.
    let mut options = Options::new();
    options.optopt("b", "batch-size",
                   &format!("Size of retweet batches. [Default: {default}]",
                            default = default_batch_size),
                   "SIZE");
    options.optopt("f", "hostfile", "A text file specifying \"hostname:port\" per line in order of process identity.", "FILE");
    options.optflag("h", "help", "Show this usage message and exit.");
    options.optopt("n", "processes", "Number of processes involved in the computation.", "PROCESSES");
    options.optopt("p", "process", "Identity of this process; from 0 to n-1.", "ID");
    options.optflag("r", "print-result", "Print the computed influence edges to stdout.");
    options.optopt("w", "workers", "Number of per-process workers threads.", "WORKERS");

    // Parse the options.
    let specified_options: Matches = match options.parse(&arguments[1..]) {
        Ok(option_matches) => option_matches,
        Err(message) => {
            println!("Error: {error}", error = message.to_string());
            print_usage(program, options);
            process::exit(ExitCode::IncorrectUsage as i32);
        }
    };

    // If the help option is specified, show the usage information and exit the program.
    if specified_options.opt_present("h") {
        print_usage(program, options);
        process::exit(ExitCode::Success as i32);
    }

    // Get the parsed options.
    let print_result: bool = specified_options.opt_present("r");
    let batch_size: usize = match specified_options.opt_str("b") {
        Some(batch_size) => {
            match batch_size.parse::<usize>() {
                Ok(batch_size) => batch_size,
                Err(message) => {
                    println!("Error: {error}", error = message.to_string());
                    print_usage(&program, options);
                    process::exit(ExitCode::IncorrectUsage as i32);
                }
            }
        },
        None => default_batch_size
    };

    // Create the arguments for the timely execution.
    let mut timely_arguments: Vec<String> = Vec::new();
    if let Some(hostfile) = specified_options.opt_str("f") {
        timely_arguments.push("-h".to_string());
        timely_arguments.push(hostfile.to_string());
    }
    if let Some(processes) = specified_options.opt_str("n") {
        timely_arguments.push("-n".to_string());
        timely_arguments.push(processes.to_string());
    }
    if let Some(process) = specified_options.opt_str("p") {
        timely_arguments.push("-p".to_string());
        timely_arguments.push(process.to_string());
    }
    if let Some(workers) = specified_options.opt_str("w") {
        timely_arguments.push("-w".to_string());
        timely_arguments.push(workers.to_string());
    }
    let timely_arguments = timely_arguments.into_iter();

    // Get the paths to the data sets.
    if specified_options.free.len() < 2 {
        let message: &str = match specified_options.free.len() {
            0 => "Neither friends data set nor retweet data set are given.",
            1 => "No retweet data set is given.",
            _ => ""
        };
        println!("Error: {error}", error = message.to_string());
        print_usage(program, options);
        process::exit(ExitCode::IncorrectUsage as i32);
    }
    let friendship_dataset: String = specified_options.free[0].clone();
    let retweet_dataset: String = specified_options.free[1].clone();

    // Execute the reconstruction.
    execute(friendship_dataset, retweet_dataset, batch_size, print_result, timely_arguments);
    process::exit(ExitCode::Success as i32);
}

/// Print a usage message.
fn print_usage(program: &str, options: Options) {
    let brief = &format!("Usage: {executable} <options> [FRIENDS] [RETWEETS]", executable = program);
    println!("{}", options.usage(brief));
}

/// Execute the reconstruction.
fn execute<I>(friendship_dataset: String, retweet_dataset: String, batch_size: usize, print_result: bool, timely_args: I)
    where I: Iterator<Item=String> {
    timely::execute_from_args(timely_args, move |computation| {
        let index = computation.index();
        let mut stopwatch = Stopwatch::start_new();

        /******************
         * DATAFLOW GRAPH *
         ******************/

        // Reconstruct the cascade.
        // Algorithm:
        // 1. Send all friendship edges (u1 -> u2, u1 follows u2) to respective workers (based on u1).
        // 2. Broadcast the current retweet r* to all workers.
        // 3. Each worker marks the user u* of r* as activated for the retweet's cascade.
        // 4. The worker storing u*'s friends produces the influence edges:
        //    a. If u* has more friends than there are activated users for this cascade, iterate
        //       over the cascade's activations. Otherwise, iterate over u*'s friends.
        //    b. For the current user u in the iteration, produce an influence edge if:
        //       i.   For activation iteration: u is a friend of u*, and
        //       ii.  (The retweet occurred after the activation of u, or
        //       iii. u is the poster of the original tweet).
        let (mut graph_input, mut retweet_input, probe) = computation.scoped::<u64, _, _>(move |scope| {

            // Create the inputs.
            let (graph_input, graph_stream) = scope.new_input();
            let (retweet_input, retweet_stream) = scope.new_input();

            let probe = retweet_stream
                .broadcast()
                .reconstruct(graph_stream)
                .inspect(move |influence: &InfluenceEdge<u64>| {
                    if print_result {
                        println!("Worker {worker}: {influencer} -> {influencee} at time {time} (cascade {cascade})",
                                 worker = index, influencer = influence.influencer,
                                 influencee = influence.influencee, time = influence.timestamp,
                                 cascade = influence.cascade_id);
                    };
                })
                .probe().0;

            (graph_input, retweet_input, probe)
        });
        let time_to_setup: u64 = stopwatch.lap();



        /****************
         * SOCIAL GRAPH *
         ****************/

        // Load the social graph from a file into the computation (only on the first worker).
        let mut number_of_friendships: u64 = 0;
        if index == 0 {
            let friendship_file = File::open(&friendship_dataset).expect("Could not open friendship dataset.");
            let friendship_file = BufReader::new(friendship_file);

            // Each line contains all friendships of a single user.
            for user in friendship_file.lines().filter_map(|u| u.ok()) {
                let user: Vec<&str> = user.split(':').collect();
                if user.len() == 0 {
                    continue;
                }

                let user_id: u64 = user[0].parse().unwrap();

                let has_friends = user.len() > 1 && !user[1].is_empty();
                if !has_friends {
                    continue;
                }

                for friend_id in user[1].split(',').map(|f| f.parse::<u64>().unwrap()) {
                    number_of_friendships += 1;
                    graph_input.send(DirectedEdge::new(user_id, friend_id));
                }
            }
        }

        // Process the entire social graph before continuing.
        computation.sync(&probe, &mut graph_input, &mut retweet_input);
        let time_to_process_social_network: u64 = stopwatch.lap();



        /************
         * RETWEETS *
         ************/

        // Load the retweets (on the first worker).
        let retweets: Vec<Tweet> = if index == 0 {
            let retweet_file = File::open(&retweet_dataset).expect("Could not open retweet file.");
            let retweet_file = BufReader::new(retweet_file);
            retweet_file.lines()
                .map(|r| serde_json::from_str::<Tweet>(&r.expect("{}")).unwrap())
                .collect()
        } else {
            Vec::new()
        };
        let number_of_retweets: usize = retweets.len();
        let time_to_load_retweets: u64 = stopwatch.lap();

        // Process the retweets.
        let mut round = 0;
        for retweet in retweets {
            retweet_input.send(retweet);

            let is_batch_complete: bool = round % batch_size == (batch_size - 1);
            if is_batch_complete {
                computation.sync(&probe, &mut retweet_input, &mut graph_input);
            }

            round += 1;
        }
        computation.sync(&probe, &mut retweet_input, &mut graph_input);
        let time_to_process_retweets: u64 = stopwatch.lap();



        /***********
         * RESULTS *
         ***********/

        stopwatch.stop();
        if index == 0 {
            println!();
            println!("Results:");
            println!("  #Friendships: {}", number_of_friendships);
            println!("  #Retweets: {}", number_of_retweets);
            println!("  Batch Size: {}", batch_size);
            println!();
            println!("  Time to set up the computation: {:.3}ms",
                     time_to_setup as f64 / 1_000_000.0f64);
            println!("  Time to load and process the social network: {:.3}ms",
                     time_to_process_social_network as f64 / 1_000_000.0f64);
            println!("  Time to load the retweets: {:.3}ms",
                     time_to_load_retweets as f64 / 1_000_000.0f64);
            println!("  Time to process the retweets: {:.3}ms",
                     time_to_process_retweets as f64 / 1_000_000.0f64);
            println!("  Total time: {:.3}ms",
                     stopwatch.total_time() as f64 / 1_000_000.0f64);
            println!();
            println!("  Retweet Processing Rate: {:.3} RT/s",
                     number_of_retweets as f64 / (time_to_process_retweets as f64 / 1_000_000_000.0f64))
        }
    }).unwrap();
}
