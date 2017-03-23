//! Execute the graph-parallel cascade reconstruction from the command line.
//!
//! This binary executes the graph-parallel Retweet cascade reconstruction algorithm. See `README.md` for more details
//! on how to run `CRGP` or call
//!
//! ```bash
//! $ cargo run --release -- -h
//! ```
//!
//! for a full list of usage information.

#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]

#[macro_use]
extern crate clap;
extern crate crgplib;

use std::process;

use clap::{Arg, ArgMatches};

use crgplib::Error;
use crgplib::algorithm;

/// The exit codes returned by the program.
#[derive(Clone, Copy, Debug)]
pub enum ExitCode {
    /// Successful (i.e. expected) execution (Code: `0`).
    Success = 0,

    /// Invalid program parameters (Code: `1`).
    IncorrectUsage = 1,

    /// Failure due to I/O operations (Code: `2`).
    IOFailure = 2,

    /// Execution failure (Code: `3`).
    ExecutionFailure = 3
}

/// Execute the program.
fn main() {
    // Define the usage.
    let arguments: ArgMatches = app_from_crate!()
        .arg(Arg::with_name("batch-size")
            .short("b")
            .long("batch-size")
            .value_name("SIZE")
            .help("Size of retweet batches")
            .takes_value(true)
            .default_value("500")
            .validator(|value: String| -> Result<(), String> {
                // The batch size must be a positive integer.
                let is_valid = match value.parse::<usize>() {
                    Ok(value) => value > 0,
                    Err(_) => false
                };

                if !is_valid {
                    Err(String::from("The batch size must be a positive integer."))
                }
                else {
                    Ok(())
                }
            }))
        .arg(Arg::with_name("hostfile")
            .short("f")
            .long("hostfile")
            .value_name("FILE")
            .help("A text file specifying \"hostname:port\" per line in order of process identity")
            .takes_value(true))
        .arg(Arg::with_name("processes")
            .short("n")
            .long("processes")
            .value_name("PROCESSES")
            .help("Number of processes involved in the computation")
            .takes_value(true))
        .arg(Arg::with_name("process")
            .short("p")
            .long("process")
            .value_name("ID")
            .help("Identity of this process; from 0 to n-1")
            .takes_value(true))
        .arg(Arg::with_name("print-results")
            .short("r")
            .long("print-result")
            .help("Prints the computed influence edges to STDOUT"))
        .arg(Arg::with_name("workers")
            .short("w")
            .long("workers")
            .value_name("WORKERS")
            .help("Number of per-process worker threads")
            .takes_value(true))
        .arg(Arg::with_name("FRIENDS")
            .help("Path to the friendship dataset")
            .required(true)
            .index(1))
        .arg(Arg::with_name("RETWEETS")
            .help("Path to the Retweet dataset")
            .required(true)
            .index(2))
        .get_matches();

    // Get the arguments. Since these arguments have default values and validators defined none of the `unwrap()`s
    // can fail.
    let batch_size: usize = arguments.value_of("batch-size").unwrap().parse().unwrap();
    let print_result: bool = arguments.is_present("print-results");

    // Get the positional arguments. Since they are required the `unwrap()`s cannot fail.
    let friendship_dataset: String = arguments.value_of("FRIENDS").unwrap().to_owned();
    let retweet_dataset: String = arguments.value_of("RETWEETS").unwrap().to_owned();

    // Create the arguments for the timely execution.
    let mut timely_arguments: Vec<String> = Vec::new();
    if let Some(hostfile) = arguments.value_of("hostfile") {
        timely_arguments.push("-h".to_owned());
        timely_arguments.push(hostfile.to_owned());
    }
    if let Some(processes) = arguments.value_of("processes") {
        timely_arguments.push("-n".to_owned());
        timely_arguments.push(processes.to_owned());
    }
    if let Some(process) = arguments.value_of("process") {
        timely_arguments.push("-p".to_owned());
        timely_arguments.push(process.to_owned());
    }
    if let Some(workers) = arguments.value_of("workers") {
        timely_arguments.push("-w".to_owned());
        timely_arguments.push(workers.to_owned());
    }
    let timely_arguments: std::vec::IntoIter<String> = timely_arguments.into_iter();

    // Execute the reconstruction.
    let results = algorithm::execute(friendship_dataset, retweet_dataset, batch_size, print_result, timely_arguments);

    // Print the statistics.
    match results {
        Ok(results) => {
            println!();
            println!("Results:");
            println!("  #Friendships: {}", results.number_of_friendships());
            println!("  #Retweets: {}", results.number_of_retweets());
            println!("  Batch Size: {}", results.batch_size());
            println!();
            println!("  Time to set up the computation: {:.3}ms", results.time_to_setup() as f64 / 1_000_000.0f64);
            println!("  Time to load and process the social network: {:.3}ms",
                     results.time_to_process_social_graph() as f64 / 1_000_000.0f64);
            println!("  Time to load the retweets: {:.3}ms", results.time_to_load_retweets() as f64 / 1_000_000.0f64);
            println!("  Time to process the retweets: {:.3}ms",
                     results.time_to_process_retweets() as f64 / 1_000_000.0f64);
            println!("  Total time: {:.3}ms", results.total_time() as f64 / 1_000_000.0f64);
            println!();
            println!("  Retweet Processing Rate: {} RT/s", results.retweet_processing_rate());

            process::exit(ExitCode::Success as i32);
        },
        Err(error) => {
            print!("Error: ");
            match error {
                Error::IO(message) => {
                    println!("{}", message);
                    process::exit(ExitCode::IOFailure as i32);
                },
                Error::Timely(message) => {
                    println!("{}", message);
                    process::exit(ExitCode::ExecutionFailure as i32);
                }
            }
        }
    };
}
