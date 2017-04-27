// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The actual algorithm for reconstructing retweet cascades.

use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;
use std::io::prelude::*;
use std::path::PathBuf;
use std::result::Result as StdResult;

use fine_grained::Stopwatch;
use regex::Regex;
use serde_json;
use tar::Archive;
use timely::dataflow::operators::Broadcast;
use timely::dataflow::operators::Input;
use timely::dataflow::operators::Probe;
use timely::execute::execute as timely_execute;
use timely::dataflow::scopes::Scope;
use timely_communication::initialize::Configuration as TimelyConfiguration;
use timely_communication::initialize::WorkerGuards;

use Configuration;
use Error;
use Result;
use Statistics;
use timely_extensions::Sync;
use timely_extensions::operators::OutputTarget;
use timely_extensions::operators::Reconstruct;
use timely_extensions::operators::Write;
use twitter::Tweet;

/// An alias for user IDs to improve code legibility.
///
/// If the stored value is negative, the ID belongs to a dummy user who was created to pad the social graph.
pub type UserID = i64;

lazy_static! {
    /// A regular expression to validate directory names. The name must consist of exactly three digits.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    pub static ref DIRECTORY_NAME_TEMPLATE: Regex = Regex::new(r"^\d{3}$").unwrap();

    /// A regular expression to validate TAR file names. The name must consist of exactly two digits followed by the
    /// extension `.tar`.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    pub static ref TAR_NAME_TEMPLATE: Regex = Regex::new(r"^\d{2}\.tar$").unwrap();

    /// A regular expression to validate file names. The name must be of the form `friends[ID].csv` where `[ID]`
    /// consists of one or more digits.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    pub static ref FILENAME_TEMPLATE: Regex = Regex::new(r"^\d{3}/\d{3}/friends\d+\.csv$").unwrap();
}

/// Execute the algorithm.
#[allow(unused_qualifications)]
pub fn execute(mut configuration: Configuration) -> Result<Statistics> {

    let timely_configuration: TimelyConfiguration = configuration.get_timely_configuration()?;
    let result: WorkerGuards<Result<Statistics>> = timely_execute(timely_configuration, move |computation| -> Result<Statistics> {
        let index = computation.index();
        let mut stopwatch = Stopwatch::start_new();

        // Log the algorithm configuration.
        info!("{:?}", configuration);

        /******************
         * DATAFLOW GRAPH *
         ******************/

        // Clone the variable so we can use it in the next closure.
        let output_target: OutputTarget = configuration.output_target.clone();

        // Reconstruct the cascade.
        // Algorithm:
        // 1. Send all friendship edges (u1 -> u2, u1 follows u2) to respective workers (based on u1).
        // 2. Broadcast the current retweet r* to all workers.
        // 3. Each worker marks the user u* of r* as activated for the retweet's cascade.
        // 4. The worker storing u*'s friends produces the influence edges:
        //    a. If u* has more friends than there are activated users for this cascade, iterate over the cascade's
        //       activations. Otherwise, iterate over u*'s friends.
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
                .write(output_target)
                .probe().0;

            (graph_input, retweet_input, probe)
        });
        let time_to_setup: u64 = stopwatch.lap();



        /****************
         * SOCIAL GRAPH *
         ****************/

        // Load the social graph into the computation (only on the first worker).
        let mut total_number_of_actual_friendship: u64 = 0;
        let mut total_number_of_friendships_given: u64 = 0;
        let mut number_of_users: u64 = 0;
        if index == 0 {
            info!("Loading social graph...");
            // Top level.
            for root_entry in read_dir(&configuration.social_graph)? {
                let path: PathBuf = match root_entry {
                    Ok(entry) => entry.path(),
                    Err(_) => continue
                };

                // The entry must be a directory.
                if !path.is_dir() {
                    continue;
                }

                // Get the last part of the path (e.g. `ZZZ` from `/xxx/yyy/ZZZ`).
                let path_c: PathBuf = path.clone();
                let directory: &str = match path_c.file_stem() {
                    Some(directory) => {
                        match directory.to_str() {
                            Some(directory) => directory,
                            None => continue
                        }
                    },
                    None => continue
                };

                // Validate the name.
                if !DIRECTORY_NAME_TEMPLATE.is_match(directory) {
                    trace!("Invalid directory name: {name:?}", name = path);
                    continue;
                }

                // TAR archives.
                for archive_entry in read_dir(path)? {
                    let path: PathBuf = match archive_entry {
                        Ok(entry) => entry.path(),
                        Err(_) => continue
                    };

                    // The entry must be a file.
                    if !path.is_file() {
                        continue;
                    }

                    // Get the file name.
                    let path_c: PathBuf = path.clone();
                    let filename: &str = match path_c.file_name() {
                        Some(filename) => {
                            match filename.to_str() {
                                Some(filename) => filename,
                                None => continue
                            }
                        },
                        None => continue
                    };

                    // Validate the name.
                    if !TAR_NAME_TEMPLATE.is_match(filename) {
                        trace!("Invalid filename: {name:?}", name = path);
                        continue;
                    }

                    // Open the archive.
                    let archive_file = match File::open(path) {
                        Ok(file) => file,
                        Err(message) => {
                            error!("Could not open archive {archive:?}: {error}",
                                   archive = path_c, error = message);
                            continue;
                        }
                    };
                    let mut archive = Archive::new(archive_file);
                    let archive_entries = match archive.entries() {
                        Ok(entries) => entries,
                        Err(message) => {
                            error!("Could not read contents of archive {archive:?}: {error}",
                                   archive = path_c, error = message);
                            continue;
                        }
                    };
                    for file in archive_entries {
                        // Ensure correct reading.
                        let file = match file {
                            Ok(file) => file,
                            Err(message) => {
                                error!("Could not read archived file in archive {archive:?}: {error}",
                                       archive = path_c, error = message);
                                continue;
                            }
                        };

                        let path: PathBuf = match file.path() {
                            Ok(path) => path.to_path_buf(),
                            Err(_) => continue
                        };
                        // Validate the filename.
                        match path.to_str() {
                            Some(path) => {
                                if !FILENAME_TEMPLATE.is_match(path) {
                                    trace!("Invalid filename: {name:?}", name = path);
                                    continue;
                                }
                            },
                            None => continue
                        }

                        // Get the user ID.
                        let user: UserID = match path.file_stem() {
                            Some(stem) => {
                                match stem.to_str() {
                                    Some(stem) => {
                                        // `stem` is now `friends[ID]`. Only parse `[ID]`, i.e. skip the first seven
                                        // characters.
                                        match stem[7..].parse::<UserID>() {
                                            Ok(id) => id,
                                            Err(message) => {
                                                warn!("Could not parse user ID '{id}': {error}",
                                                      id = &stem[7..], error = message);
                                                continue;
                                            }
                                        }
                                    },
                                    None => continue
                                }
                            },
                            None => continue
                        };

                        // Parse the file.
                        let reader = BufReader::new(file);
                        let mut is_first_line: bool = true;
                        let mut actual_number_of_friends: u64 = 0;
                        let friendships: Vec<UserID> = reader.lines()
                            .filter_map(|line: IOResult<String>| -> Option<String> {
                                // Ensure correct encoding.
                                match line {
                                    Ok(line) => Some(line),
                                    Err(message) => {
                                        warn!("Invalid line in file {file:?}: {error}", file = path, error = message);
                                        None
                                    }
                                }
                            })
                            .filter_map(|line: String| -> Option<UserID> {
                                // Parse the friend ID.
                                match line.parse::<UserID>() {
                                    Ok(id) => Some(id),
                                    Err(message) => {
                                        // If this is the first line in the file, it may contain meta data.
                                        let meta_data: Vec<&str> = if is_first_line {
                                            is_first_line = false;
                                            line.split(';')
                                                .collect()
                                        } else {
                                            Vec::new()
                                        };

                                        // Index 3 contains the actual number of friendships.
                                        if meta_data.len() == 5 {
                                            match meta_data[3].parse() {
                                                Ok(amount) => {
                                                    actual_number_of_friends = amount;
                                                    return None;
                                                },
                                                Err(_) => {}
                                            }
                                        }

                                        warn!("Could not parse friend ID '{friend}' of user {user}: {error}",
                                              friend = line, user = user, error = message);
                                        None
                                    }
                                }
                            })
                            .collect();

                        if friendships.len() == 0 {
                            warn!("User {user} does not have any friends", user = user);
                            continue;
                        }

                        // Send the friendships.
                        let given_number_of_friends: u64 = friendships.len() as u64;
                        trace!("User {user}: {given} of {actual} friends found", user = user,
                               given = given_number_of_friends, actual = actual_number_of_friends);

                        total_number_of_friendships_given += given_number_of_friends;
                        total_number_of_actual_friendship += actual_number_of_friends;
                        number_of_users += 1;
                        graph_input.send((user, friendships));
                    }
                }
            }
        }

        // Process the entire social graph before continuing.
        computation.sync(&probe, &mut graph_input, &mut retweet_input);
        let time_to_process_social_network: u64 = stopwatch.lap();
        info!("Finished loading the social graph in {time:.2}ms",
              time = time_to_process_social_network as f64 / 1_000_000.0f64);
        info!("Found {given} of {actual} friendships in the data set for {users} users",
              given = total_number_of_friendships_given, actual = total_number_of_actual_friendship,
              users = number_of_users);



        /************
         * RETWEETS *
         ************/

        // Load the retweets (on the first worker).
        let retweets: Vec<Tweet> = if index == 0 {
            let path = PathBuf::from(&configuration.retweets);
            if !path.is_file() {
                error!("Retweet data set is a not a file");
                return Err(Error::from(IOError::new(IOErrorKind::InvalidInput, "Retweet data set is not a file")));
            }

            info!("Loading Retweets");
            let retweet_file = match File::open(&path) {
                Ok(file) => file,
                Err(error) => {
                    error!("Could not open Retweet data set: {error}", error = error);
                    return Err(Error::from(error));
                }
            };
            let retweet_file = BufReader::new(retweet_file);
            // Parse the lines while discarding those that are invalid.
            retweet_file.lines()
                .filter_map(|line: IOResult<String>| -> Option<Tweet> {
                    match line {
                        Ok(line) => {
                            match serde_json::from_str::<Tweet>(&line) {
                                Ok(tweet) => return Some(tweet),
                                Err(message) => {
                                    warn!("Failed to parse Tweet: {error}", error = message);
                                    return None;
                                }
                            }
                        },
                        Err(message) => {
                            warn!("Invalid line in file {file:?}: {error}",
                                  file = configuration.retweets, error = message);
                            return None;
                        }
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        let number_of_retweets: u64 = retweets.len() as u64;
        let time_to_load_retweets: u64 = stopwatch.lap();
        info!("Finished loading Retweets in {time:.2}ms",
              time = time_to_load_retweets as f64 / 1_000_000.0f64);

        // Process the retweets.
        info!("Processing Retweets");
        let batch_size: usize = configuration.batch_size;
        let mut round = 0;
        for retweet in retweets {
            retweet_input.send(retweet);

            let is_batch_complete: bool = round % batch_size == (batch_size - 1);
            if is_batch_complete {
                trace!("Processed {amount} of {total} Retweets...", amount = round + 1, total = number_of_retweets);
                computation.sync(&probe, &mut retweet_input, &mut graph_input);
            }

            round += 1;
        }
        computation.sync(&probe, &mut retweet_input, &mut graph_input);
        let time_to_process_retweets: u64 = stopwatch.lap();
        info!("Finished processing {amount} Retweets in {time:.2}ms", amount = number_of_retweets,
              time = time_to_process_retweets as f64 / 1_000_000.0f64);



        /**********
         * FINISH *
         **********/

        stopwatch.stop();
        Ok(Statistics::new(total_number_of_friendships_given, number_of_retweets, batch_size, time_to_setup,
                           time_to_process_social_network, time_to_load_retweets, time_to_process_retweets,
                           stopwatch.total_time()))
    })?;

    // The result returned from the computation is several layers of nested Result types. Flatten them to the expected
    // return type. Return the statistics from the first worker, but only if no worker returned an error.
    let worker_results: Vec<(usize, Result<Statistics>)> = result.join()
        .into_iter()
        .map(|worker_result: StdResult<Result<Statistics>, String>| {  // Flatten the nested result types.
            match worker_result {
                Ok(result) => {
                    match result {
                        Ok(statistics) => Ok(statistics),
                        Err(error) => Err(error)
                    }
                },
                Err(message) => Err(Error::from(message))
            }
        })
        .enumerate()
        .rev()
        .collect();

    // The worker results have been enumerated with their worker's index, and this list has been reversed, i.e. the
    // first worker is now at the end. For all workers but the first one, immediately return their failure as this
    // function's return value if they failed. If none of these workers failed return the result of the first worker.
    for (index, worker_result) in worker_results {
        if index == 0 {
            return worker_result
        }
        else {
            match worker_result {
                Ok(_) => continue,
                Err(_) => return worker_result
            }
        }
    }

    // This could only happen if there were no workers at all.
    Err(Error::from("No workers".to_string()))
}
