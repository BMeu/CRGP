// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

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
extern crate flexi_logger;

use std::env::current_dir;
use std::path::PathBuf;
use std::process;

use clap::{Arg, ArgMatches};
use flexi_logger::{LogOptions, with_thread};

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

    /// Failure during logger initialization (Code: `3`).
    LoggerFailure = 3,

    /// Execution failure (Code: `4`).
    ExecutionFailure = 4,
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
                match value.parse::<usize>() {
                    Ok(value) if value > 0 => Ok(()),
                    _ => Err(String::from("The batch size must be a positive integer."))
                }
            }))
        .arg(Arg::with_name("hostfile")
            .short("f")
            .long("hostfile")
            .value_name("FILE")
            .help("A text file specifying \"hostname:port\" per line in order of process identity")
            .takes_value(true))
        .arg(Arg::with_name("log")
            .short("l")
            .long("log-directory")
            .value_name("DIRECTORY")
            .help("The directory where log files will be created (if logging is enabled via '-v'). If this argument is \
                  not specified log messages will be written to STDERR.")
            .takes_value(true))
        .arg(Arg::with_name("processes")
            .short("n")
            .long("processes")
            .value_name("PROCESSES")
            .help("Number of processes involved in the computation")
            .takes_value(true))
        .arg(Arg::with_name("output-directory")
            .short("o")
            .long("output-directory")
            .value_name("DIRECTORY")
            .help("The directory where the result and statistics files will be created. If this argument is not \
                  specified the current direcotry will be used.")
            .takes_value(true))
        .arg(Arg::with_name("process")
            .short("p")
            .long("process")
            .value_name("ID")
            .help("Identity of this process; from 0 to n-1")
            .takes_value(true))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Sets the log level. Without this argument, logging will be disabled. The argument can occur \
                  multiple times."))
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

    // Get the positional arguments. Since they are required the `unwrap()`s cannot fail.
    let friendship_dataset: String = arguments.value_of("FRIENDS").unwrap().to_owned();
    let retweet_dataset: String = arguments.value_of("RETWEETS").unwrap().to_owned();

    // Create the arguments for the timely execution.
    let mut timely_arguments: Vec<String> = Vec::new();
    if let Some(hostfile) = arguments.value_of("hostfile") {
        timely_arguments.push("-h".to_owned());
        timely_arguments.push(hostfile.to_owned());
    }
    let (log_to_file, log_directory): (bool, Option<String>) = match arguments.value_of("log") {
        Some(directory) => (true, Some(String::from(directory))),
        None => (false, None)
    };
    let output_directory: Option<PathBuf> = match arguments.value_of("output-directory") {
        Some(directory) => Some(PathBuf::from(directory)),
        None => match current_dir() {
            Ok(directory) => Some(directory),
            Err(error) => {
                println!("Error: {message}", message = error);
                process::exit(ExitCode::IOFailure as i32);
            }
        },
    };
    if let Some(processes) = arguments.value_of("processes") {
        timely_arguments.push("-n".to_owned());
        timely_arguments.push(processes.to_owned());
    }
    if let Some(process) = arguments.value_of("process") {
        timely_arguments.push("-p".to_owned());
        timely_arguments.push(process.to_owned());
    }
    let verbosity: Option<String> = match arguments.occurrences_of("verbosity") {
        0 => None,
        1 => Some(String::from("error")),
        2 => Some(String::from("warn")),
        3 => Some(String::from("info")),
        4 | _ => Some(String::from("trace"))
    };
    if let Some(workers) = arguments.value_of("workers") {
        timely_arguments.push("-w".to_owned());
        timely_arguments.push(workers.to_owned());
    }
    let timely_arguments: std::vec::IntoIter<String> = timely_arguments.into_iter();

    // Initialize the logger.
    if let Some(verbosity) = verbosity {
        let logger_initialization = LogOptions::new()
            .format(with_thread)
            .log_to_file(log_to_file)
            .directory(log_directory)
            .init(Some(verbosity));

        match logger_initialization {
            Ok(_) => {},
            Err(error) => {
                println!("Error: {message}", message = error);
                process::exit(ExitCode::LoggerFailure as i32);
            }
        }
    }

    // Execute the algorithm.
    let results = algorithm::execute(friendship_dataset, retweet_dataset, batch_size, output_directory, timely_arguments);

    // Print the statistics.
    match results {
        Ok(results) => {
            println!();
            println!("Results:");
            println!("  #Friendships: {}", results.number_of_friendships());
            println!("  #Retweets: {}", results.number_of_retweets());
            println!("  Batch Size: {}", results.batch_size());
            println!();
            println!("  Time to set up the computation: {:.2}ms", results.time_to_setup() as f64 / 1_000_000.0f64);
            println!("  Time to load and process the social network: {:.2}ms",
                     results.time_to_process_social_graph() as f64 / 1_000_000.0f64);
            println!("  Time to load the retweets: {:.2}ms", results.time_to_load_retweets() as f64 / 1_000_000.0f64);
            println!("  Time to process the retweets: {:.2}ms",
                     results.time_to_process_retweets() as f64 / 1_000_000.0f64);
            println!("  Total time: {:.2}ms", results.total_time() as f64 / 1_000_000.0f64);
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
