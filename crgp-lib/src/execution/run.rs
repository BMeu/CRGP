// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Run the reconstruction.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use fine_grained::Stopwatch;
use timely::dataflow::operators::Broadcast;
use timely::dataflow::operators::Filter;
use timely::dataflow::operators::Input;
use timely::dataflow::operators::Probe;
use timely::dataflow::operators::exchange::Exchange;
use timely::execute::execute as timely_execute;
use timely::dataflow::scopes::Scope;
use timely_communication::initialize::Configuration as TimelyConfiguration;
use timely_communication::initialize::WorkerGuards;

use Algorithm;
use Configuration;
use OutputTarget;
use Result;
use Statistics;
use UserID;
use execution::simplify_result::SimplifyResult;
use social_graph::InfluenceEdge;
use social_graph::source::tar;
use timely_extensions::Sync;
use timely_extensions::operators::FindPossibleInfluences;
use timely_extensions::operators::Reconstruct;
use timely_extensions::operators::Write;
use twitter;
use twitter::Tweet;

/// Execute the algorithm.
pub fn run(mut configuration: Configuration) -> Result<Statistics> {

    let timely_configuration: TimelyConfiguration = configuration.get_timely_configuration()?;
    let result: WorkerGuards<Result<Statistics>> = timely_execute(timely_configuration,
                                                                  move |computation| -> Result<Statistics> {
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
        let (mut graph_input, mut retweet_input, probe) = match configuration.algorithm {
            Algorithm::LEAF => {
                // Algorithm:
                // 1. Send all friendship edges (u1 -> u2, u1 follows u2) to respective workers (based on u1).
                // 2. Send a retweet made by u* to the worker storing u*'s friendships.
                // 3. On this worker: mark u* and the original user u as active for this cascade.
                // 4. On this worker: for all friends of u*, create (possible) influence edges (PIE) for this
                //    cascade, from the friend u' to u*, with timestamp of u*'s retweet.
                // 5. Send each PIE to the worker storing u'.
                // 6. On this worker: filter all PIEs, output only those where u' has been activated before.
                computation.scoped::<u64, _, _>(move |scope| {
                    // Create the inputs.
                    let (graph_input, graph_stream) = scope.new_input();
                    let (retweet_input, retweet_stream) = scope.new_input();

                    // For each cascade, given by its ID, a set of activated users, given by their ID, i.e.
                    // those users who have retweeted within this cascade before, per worker. Since this map
                    // is required within two closures, dynamic borrow checks are required.
                    let activations_influences: Rc<RefCell<HashMap<u64, HashMap<UserID, u64>>>> =
                        Rc::new(RefCell::new(HashMap::new()));
                    let activations_possible_influences = activations_influences.clone();

                    let probe = graph_stream
                        .find_possible_influences(retweet_stream, activations_possible_influences)
                        .exchange(|influence: &InfluenceEdge<UserID>| influence.influencer as u64)
                        .filter(move |influence: &InfluenceEdge<UserID>| {
                            let is_influencer_activated: bool = match activations_influences.borrow()
                                .get(&influence.cascade_id)
                                {
                                    Some(users) => match users.get(&influence.influencer) {
                                        Some(activation_timestamp) => &influence.timestamp > activation_timestamp,
                                        None => false
                                    },
                                    None => false
                                };
                            let is_influencer_original_user: bool = influence.influencer == influence.original_user;

                            is_influencer_activated || is_influencer_original_user
                        })
                        .write(output_target)
                        .probe().0;

                    (graph_input, retweet_input, probe)
                })
            },
            Algorithm::GALE => {
                // Algorithm:
                // 1. Send all friendship edges (u1 -> u2, u1 follows u2) to respective workers (based on u1).
                // 2. Broadcast the current retweet r* to all workers.
                // 3. Each worker marks the user u* of r* as activated for the retweet's cascade.
                // 4. The worker storing u*'s friends produces the influence edges:
                //    a. If u* has more friends than there are activated users for this cascade, iterate over the
                //       cascade's activations. Otherwise, iterate over u*'s friends.
                //    b. For the current user u in the iteration, produce an influence edge if:
                //       i.   For activation iteration: u is a friend of u*, and
                //       ii.  (The retweet occurred after the activation of u, or
                //       iii. u is the poster of the original tweet).
                computation.scoped::<u64, _, _>(move |scope| {
                    // Create the inputs.
                    let (graph_input, graph_stream) = scope.new_input();
                    let (retweet_input, retweet_stream) = scope.new_input();

                    let probe = retweet_stream
                        .broadcast()
                        .reconstruct(graph_stream)
                        .write(output_target)
                        .probe().0;

                    (graph_input, retweet_input, probe)
                })
            }
        };
        let time_to_setup: u64 = stopwatch.lap();



        /****************
         * SOCIAL GRAPH *
         ****************/

        // Load the social graph into the computation (only on the first worker).
        let counts: (u64, u64, u64) = if index == 0 {
            info!("Loading social graph...");
            let path = PathBuf::from(configuration.social_graph.clone());
            tar::load(&path, configuration.pad_with_dummy_users, &mut graph_input)?
        } else {
            (0, 0, 0)
        };
        let (number_of_users, mut number_of_explicit_friendships, total_number_of_friendships) = counts;

        // Process the entire social graph before continuing.
        computation.sync(&probe, &mut graph_input, &mut retweet_input);
        let time_to_process_social_network: u64 = stopwatch.lap();

        // Log loading information (only on the first worker).
        if index == 0 {
            info!("Finished loading the social graph in {time}ns", time = time_to_process_social_network);
            info!("Found {given} of {actual} friendships in the data set for {users} users",
                  given = number_of_explicit_friendships, actual = total_number_of_friendships,
                  users = number_of_users);

            if configuration.pad_with_dummy_users {
                let number_of_dummy_users: u64 = total_number_of_friendships - number_of_explicit_friendships;
                info!("Created {number} dummy friends", number = number_of_dummy_users);

                // For the statistics, add the dummy friends to the size of the social graph.
                number_of_explicit_friendships += number_of_dummy_users;
            }
        }



        /************
         * RETWEETS *
         ************/

        // Load the retweets (on the first worker).
        let retweets: Vec<Tweet> = if index == 0 {
            let path = PathBuf::from(&configuration.retweets);
            twitter::get::from_file(&path)?
        } else {
            Vec::new()
        };
        let time_to_load_retweets: u64 = stopwatch.lap();

        let number_of_retweets: u64 = retweets.len() as u64;
        info!("Finished loading Retweets in {time}ns", time = time_to_load_retweets);

        // Process the retweets.
        info!("Processing Retweets");
        let batch_size: usize = configuration.batch_size;
        for (round, retweet) in retweets.iter().enumerate() {
            retweet_input.send(retweet.clone());

            // Sync the computation after each batch.
            let is_batch_complete: bool = round % batch_size == (batch_size - 1);
            if is_batch_complete {
                trace!("Processed {amount} of {total} Retweets...", amount = round + 1, total = number_of_retweets);
                computation.sync(&probe, &mut retweet_input, &mut graph_input);
            }
        }
        computation.sync(&probe, &mut retweet_input, &mut graph_input);
        let time_to_process_retweets: u64 = stopwatch.lap();

        info!("Finished processing {amount} Retweets in {time}ns", amount = number_of_retweets,
              time = time_to_process_retweets);



        /**********
         * FINISH *
         **********/

        stopwatch.stop();
        let statistics = Statistics::new(configuration.clone())
            .number_of_friendships(number_of_explicit_friendships)
            .number_of_retweets(number_of_retweets)
            .time_to_setup(time_to_setup)
            .time_to_process_social_graph(time_to_process_social_network)
            .time_to_load_retweets(time_to_load_retweets)
            .time_to_process_retweets(time_to_process_retweets)
            .total_time(stopwatch.total_time());

        // Log the statistics.
        info!("{:?}", statistics);

        Ok(statistics)
    })?;

    result.simplify()
}
