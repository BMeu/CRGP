extern crate crgp;
extern crate fine_grained;
extern crate serde_json;
extern crate timely;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use fine_grained::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::operators::*;

use crgp::social_graph::edge::*;
use crgp::timely_extensions::Sync;
use crgp::timely_extensions::operators::Reconstruct;
use crgp::twitter::*;

fn main() {
    let name = &std::env::args().nth(0).unwrap();
    println!("Usage: {} <Friend Data Set: Path> <Retweet Data Set: Path> <Batch Size: Int> <Print Result: Bool> [Timely Options]", name);
    println!();

    // Parse the arguments.
    let friendship_dataset = std::env::args().nth(1).unwrap();
    let retweet_dataset = std::env::args().nth(2).unwrap();
    let batch_size: usize = std::env::args().nth(3).unwrap().parse().unwrap();
    let print_result: bool = std::env::args().nth(4).unwrap().parse().unwrap();
    let timely_args = std::env::args().skip(4);

    execute(friendship_dataset, retweet_dataset, batch_size, print_result, timely_args);
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
            println!("  Time to set up the computation: {:.3}ms", time_to_setup as f64 / 1_000_000.0f64);
            println!("  Time to load and process the social network: {:.3}ms", time_to_process_social_network as f64 / 1_000_000.0f64);
            println!("  Time to load the retweets: {:.3}ms", time_to_load_retweets as f64 / 1_000_000.0f64);
            println!("  Time to process the retweets: {:.3}ms", time_to_process_retweets as f64 / 1_000_000.0f64);
            println!("  Total time: {:.3}ms", stopwatch.total_time() as f64 / 1_000_000.0f64);
            println!();
            println!("  Retweet Processing Rate: {:.3} RT/s", number_of_retweets as f64 / (time_to_process_retweets as f64 / 1_000_000_000.0f64))
        }
    }).unwrap();
}
