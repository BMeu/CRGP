// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Run the reconstruction.

use std::path::PathBuf;

use fine_grained::Stopwatch;
use timely::execute::execute as timely_execute;
use timely::dataflow::scopes::Scope;
use timely_communication::initialize::Configuration as TimelyConfiguration;
use timely_communication::initialize::WorkerGuards;

use Algorithm;
use Configuration;
use OutputTarget;
use Result;
use Statistics;
use reconstruction::SimplifyResult;
use reconstruction::algorithms::gale;
use reconstruction::algorithms::leaf;
use social_graph::source::tar;
use timely_extensions::Sync;
use twitter;
use twitter::Tweet;

/// Execute the reconstruction.
pub fn run(mut configuration: Configuration) -> Result<Statistics> {

    let timely_configuration: TimelyConfiguration = configuration.get_timely_configuration()?;
    let result: WorkerGuards<Result<Statistics>> = timely_execute(timely_configuration,
                                                                  move |computation| -> Result<Statistics> {
        let index = computation.index();
        let mut stopwatch = Stopwatch::start_new();

        // Log the algorithm configuration.
        info!("Configuration: {}", configuration);

        /******************
         * DATAFLOW GRAPH *
         ******************/

        // Clone parts of the configuration so we can use them in the next closure.
        let algorithm = configuration.algorithm;
        let output_target: OutputTarget = configuration.output_target.clone();

        // Reconstruct the cascade.
        let (mut graph_input, mut retweet_input, probe) = computation.scoped::<u64, _, _>(move |scope| {
            match algorithm {
                Algorithm::GALE => gale::computation(scope, output_target),
                Algorithm::LEAF => leaf::computation(scope, output_target)
            }
        });
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
        let (number_of_users, number_of_given_friendships, number_of_expected_friendships) = counts;

        // Process the entire social graph before continuing.
        computation.sync(&probe, &mut graph_input, &mut retweet_input);
        let time_to_process_social_network: u64 = stopwatch.lap();

        // Log loading information (only on the first worker).
        let friendships_in_social_graph: u64 = if index == 0 {
            info!("Finished loading the social graph in {time}ns", time = time_to_process_social_network);
            info!("Found {given} of {actual} friendships in the data set for {users} users",
                  given = number_of_given_friendships, actual = number_of_expected_friendships,
                  users = number_of_users);

            let mut friendships_in_social_graph: u64 = number_of_given_friendships;
            if configuration.pad_with_dummy_users {
                let number_of_dummy_users: u64 = number_of_expected_friendships - number_of_given_friendships;
                info!("Created {number} dummy friends", number = number_of_dummy_users);

                // For the statistics, add the dummy friends to the size of the social graph.
                friendships_in_social_graph += number_of_dummy_users;
            }
            friendships_in_social_graph
        } else {
            0
        };



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
            .number_of_friendships(friendships_in_social_graph)
            .number_of_retweets(number_of_retweets)
            .time_to_setup(time_to_setup)
            .time_to_process_social_graph(time_to_process_social_network)
            .time_to_load_retweets(time_to_load_retweets)
            .time_to_process_retweets(time_to_process_retweets)
            .total_time(stopwatch.total_time());

        // Log the statistics.
        info!("Statistics: {}", statistics);

        Ok(statistics)
    })?;

    result.simplify()
}
