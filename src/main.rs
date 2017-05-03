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
extern crate crgp_lib;
extern crate flexi_logger;
extern crate time;
extern crate toml;

use std::env::current_dir;
use std::error::Error as StdError;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::io::BufWriter;
use std::io::Error as IOError;
use std::path::PathBuf;

use clap::Arg;
use clap::ArgMatches;
use crgp_lib::Algorithm;
use crgp_lib::Configuration;
use crgp_lib::Error;
use crgp_lib::OutputTarget;
use flexi_logger::with_thread;
use flexi_logger::LogOptions;
use time::Tm;
use time::TmFmt;

pub use exit::ExitCode;

pub mod validation;
pub mod exit;

// TODO: Get from crate?
const PROGRAM_NAME: &str = "crgp";

/// Execute the program.
fn main() {
    // Define the usage.
    let arguments: ArgMatches = app_from_crate!()
        .arg(Arg::with_name("algorithm")
            .long("algorithm")
            .takes_value(true)
            .possible_values(&["GALE", "LEAF"])
            .default_value("GALE")
            .help("Use the specified algorithm."))
        .arg(Arg::with_name("batch-size")
            .short("b")
            .long("batch-size")
            .value_name("SIZE")
            .help("Size of retweet batches")
            .takes_value(true)
            .default_value("500")
            .validator(validation::positive_usize))
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
        .arg(Arg::with_name("pad-users")
            .long("pad-users")
            .help("If the given friend list for each user is only a subset of their friends, create as many dummy \
                  users as needed to reach the user's actual number of friends."))
        .arg(Arg::with_name("processes")
            .short("n")
            .long("processes")
            .value_name("PROCESSES")
            .help("Number of processes involved in the computation")
            .takes_value(true)
            .default_value("1")
            .validator(validation::positive_usize))
        .arg(Arg::with_name("output-directory")
            .short("o")
            .long("output-directory")
            .value_name("DIRECTORY")
            .help("The directory where the result and statistics files will be created. If this argument is not \
                  specified the current direcotry will be used.")
            .takes_value(true))
        .arg(Arg::with_name("no-output")
            .long("no-output")
            .help("Do not write any results. This setting overwrites \"--output-directory\"."))
        .arg(Arg::with_name("process")
            .short("p")
            .long("process")
            .value_name("ID")
            .help("Identity of this process; from 0 to n-1")
            .takes_value(true)
            .default_value("0")
            .validator(validation::usize))
        .arg(Arg::with_name("report-connection-progress")
            .long("report-connection-progress")
            .help("Print connection progress to STDOUT when using multiple processes."))
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
            .takes_value(true)
            .default_value("1")
            .validator(validation::positive_usize))
        .arg(Arg::with_name("FRIENDS")
            .help("Path to the friendship dataset")
            .required(true)
            .index(1))
        .arg(Arg::with_name("RETWEETS")
            .help("Path to the Retweet dataset")
            .required(true)
            .index(2))
        .get_matches();

    // Get the positional arguments. Since they are required the `unwrap()`s cannot fail.
    let social_graph_path: String = arguments.value_of("FRIENDS").unwrap().to_owned();
    let retweet_path: String = arguments.value_of("RETWEETS").unwrap().to_owned();

    // Get the arguments with default values. Since these arguments have default values and validators defined none
    // of the `unwrap()`s can fail.
    let given_algorithm: &str = arguments.value_of("algorithm").unwrap();
    let algorithm: Algorithm = if given_algorithm == "LEAF" {
        Algorithm::LEAF
    } else {
        Algorithm::GALE
    };
    let batch_size: usize = arguments.value_of("batch-size").unwrap().parse().unwrap();
    let process_id: usize = arguments.value_of("process").unwrap().parse().unwrap();
    let processes: usize = arguments.value_of("processes").unwrap().parse().unwrap();
    let workers: usize = arguments.value_of("workers").unwrap().parse().unwrap();
    let report_connection_progess: bool = arguments.is_present("report-connection-progress");
    let pad_with_dummy_users: bool = arguments.is_present("pad-users");

    // Determine the output target.
    let output_target: OutputTarget = if arguments.is_present("no-output") {
        OutputTarget::None
    } else {
        match arguments.value_of("output-directory") {
            Some(directory) => OutputTarget::Directory(PathBuf::from(directory)),
            None => match current_dir() {
                Ok(directory) => OutputTarget::Directory(directory),
                Err(error) => {
                    exit::fail_from_error(Error::from(error));
                }
            },
        }
    };
    let output_target_statistics: OutputTarget = output_target.clone();

    // Get the hosts.
    let hosts: Option<Vec<String>> = match arguments.value_of("hostfile") {
        Some(file) => {
            let file = match File::open(file) {
                Ok(file) => file,
                Err(error) => {
                    exit::fail_from_error(Error::from(error));
                }
            };
            let reader = BufReader::new(file);
            match reader.lines().collect::<Result<Vec<String>, IOError>>() {
                Ok(hosts) => Some(hosts),
                Err(error) => {
                    exit::fail_from_error(Error::from(error));
                }
            }
        },
        None => None,
    };

    // Get the logger arguments.
    let (log_to_file, log_directory): (bool, Option<String>) = match arguments.value_of("log") {
        Some(directory) => (true, Some(String::from(directory))),
        None => (false, None)
    };
    let verbosity: Option<String> = match arguments.occurrences_of("verbosity") {
        0 => None,
        1 => Some(String::from("error")),
        2 => Some(String::from("warn")),
        3 => Some(String::from("info")),
        4 | _ => Some(String::from("trace"))
    };

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
                exit::fail_with_message(ExitCode::LoggerFailure, error.description());
            }
        }
    }

    // Set the algorithm configuration.
    let configuration = Configuration::default(retweet_path, social_graph_path)
        .algorithm(algorithm)
        .batch_size(batch_size)
        .hosts(hosts)
        .output_target(output_target)
        .pad_with_dummy_users(pad_with_dummy_users)
        .process_id(process_id)
        .processes(processes)
        .report_connection_progress(report_connection_progess)
        .workers(workers);

    // Execute the algorithm.
    let results = crgp_lib::run(configuration);

    // Write the statistics.
    match results {
        Ok(results) => {
            if process_id == 0 {
                // Only save to file if output is requested.
                if let OutputTarget::Directory(directory) = output_target_statistics {
                    // Parse the statistics to TOML.
                    if let Ok(results) = toml::to_string(&results) {
                        // Create the file name from the program name and the current time.
                        let current_time: Tm = time::now();
                        // The unwrap is save, since the format string is known to be correct.
                        let time_formatted: TmFmt = current_time.strftime("%Y-%m-%d_%H-%M-%S").unwrap();
                        let filename = format!("{program}_{time}.toml", program = PROGRAM_NAME, time = time_formatted);
                        let path: PathBuf = directory.join(filename);
                        let path_c: PathBuf = path.clone();

                        // Create the file and save the results.
                        if let Ok(file) = File::create(path) {
                            let mut writer: BufWriter<File> = BufWriter::new(file);

                            // Write and flush the result.
                            let write_result = write!(writer, "{toml}", toml = results);
                            let flush_result = writer.flush();

                            if write_result.is_ok() && flush_result.is_ok() {
                                println!("Statistics saved to {path:?}", path = path_c);
                                exit::succeed();
                            }
                        }
                    }

                    // Some error occurred along the way.
                    println!("Error: could not create statistics file. Printing to STDOUT instead.");
                }

                // Writing to file failed (or was not requested) - print to STDOUT instead.
                println!();
                println!("Results:");
                println!(" #Friendships: {}", results.number_of_friendships);
                println!(" #Retweets: {}", results.number_of_retweets);
                println!();
                println!(" Time to set up the computation: {}ns", results.time_to_setup);
                println!(" Time to load and process the social network: {}ns", results.time_to_process_social_graph);
                println!(" Time to load the retweets: {}ns", results.time_to_load_retweets);
                println!(" Time to process the retweets: {}ns", results.time_to_process_retweets);
                println!(" Total time: {}ns", results.total_time);
                println!();
                println!(" Retweet Processing Rate: {} RT/s", results.retweet_processing_rate);
            }

            exit::succeed();
        },
        Err(error) => {
            exit::fail_from_error(error);
        }
    };
}
