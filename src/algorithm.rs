//! The actual algorithm for reconstructing retweet cascades.

use std::fs::File;
use std::io::BufReader;
use std::io::Result as IOResult;
use std::io::prelude::*;
use std::marker;
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::{Arc, Mutex, MutexGuard};

use fine_grained::Stopwatch;
use serde_json;
use timely;
use timely::dataflow::*;
use timely::dataflow::operators::*;
use timely_communication::initialize::WorkerGuards;

use Error;
use Result;
use Statistics;
use social_graph::*;
use timely_extensions::Sync;
use timely_extensions::operators::{Reconstruct, Write};
use twitter::*;

/// Execute the algorithm.
#[allow(unused_qualifications)]
pub fn execute<F, I>(friendships: Arc<Mutex<Option<F>>>, retweet_dataset: String, batch_size: usize,
                     output_directory: Option<PathBuf>, timely_args: I) -> Result<Statistics>
    where F: Iterator<Item=DirectedEdge<u64>> + marker::Send + marker::Sync + 'static,
          I: Iterator<Item=String> {

    let result: WorkerGuards<Result<Statistics>> = timely::execute_from_args(timely_args,
                                                                             move |computation| -> Result<Statistics> {
        let index = computation.index();
        let mut stopwatch = Stopwatch::start_new();

        /******************
         * DATAFLOW GRAPH *
         ******************/

        // Clone the variable so we can use it in the next closure.
        let output_directory_c: Option<PathBuf> = output_directory.clone();

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
                .write(output_directory_c)
                .probe().0;

            (graph_input, retweet_input, probe)
        });
        let time_to_setup: u64 = stopwatch.lap();



        /****************
         * SOCIAL GRAPH *
         ****************/

        // Load the social graph into the computation (only on the first worker).
        let mut number_of_friendships: u64 = 0;
        if index == 0 {
            info!("Loading social graph into the computation...");
            let mut friendships: MutexGuard<Option<F>> = match friendships.lock() {
                Ok(guard) => guard,
                Err(poisened) => poisened.into_inner()
            };
            let friendships: F = match friendships.take() {
                Some(friendships) => friendships,
                None => return Err(Error::from(String::from("No friendships")))
            };
            for friendship in friendships {
                number_of_friendships += 1;
                graph_input.send(friendship);
            }
        }

        // Process the entire social graph before continuing.
        computation.sync(&probe, &mut graph_input, &mut retweet_input);
        let time_to_process_social_network: u64 = stopwatch.lap();
        info!("Finished loading the social graph ({amount} friendships) in {time:.2}ms", amount = number_of_friendships,
              time = time_to_process_social_network as f64 / 1_000_000.0f64);



        /************
         * RETWEETS *
         ************/

        // Load the retweets (on the first worker).
        let retweets: Vec<Tweet> = if index == 0 {
            info!("Loading the Retweets into memory...");
            let retweet_file = match File::open(&retweet_dataset) {
                Ok(file) => file,
                Err(error) => {
                    error!("Could not open Retweet dataset: {error}", error = error);
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
                                    info!("Failed to parse tweet: {error}", error = message);
                                    return None;
                                }
                            }
                        },
                        Err(message) => {
                            warn!("Invalid line in file {file:?}: {error}", file = retweet_dataset, error = message);
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
        info!("Finished loading the Retweets into memory in {time:.2}ms",
              time = time_to_load_retweets as f64 / 1_000_000.0f64);

        // Process the retweets.
        info!("Processing the Retweets...");
        let mut round = 0;
        for retweet in retweets {
            retweet_input.send(retweet);

            let is_batch_complete: bool = round % batch_size == (batch_size - 1);
            if is_batch_complete {
                info!("Processed {amount} of {total} Retweets...", amount = round + 1, total = number_of_retweets);
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
        Ok(Statistics::new(number_of_friendships, number_of_retweets, batch_size, time_to_setup,
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
