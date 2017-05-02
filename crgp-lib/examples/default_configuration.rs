// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Execute the graph-parallel cascade reconstruction with default settings.

#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]

extern crate crgp_lib;

use crgp_lib::Configuration;
use crgp_lib::Result;
use crgp_lib::Statistics;
use crgp_lib::algorithm;

/// Execute the program.
fn main() {
    // Use the default algorithm configuration.
    let retweet_path = String::from("../data/retweets.json");
    let social_graph_path = String::from("../data/social_graph");
    let configuration = Configuration::default(retweet_path, social_graph_path);

    // Execute the algorithm.
    let result: Result<Statistics> = algorithm::execute(configuration);

    // Print the results (or an error message).
    match result {
        Ok(results) => {
            println!();
            println!("Results:");
            println!("  #Friendships: {}", results.number_of_friendships);
            println!("  #Retweets: {}", results.number_of_retweets);
            println!();
            println!("  Time to set up the computation: {:.2}ms", results.time_to_setup as f64 / 1_000_000.0f64);
            println!("  Time to load and process the social network: {:.2}ms",
                     results.time_to_process_social_graph as f64 / 1_000_000.0f64);
            println!("  Time to load the retweets: {:.2}ms",
                     results.time_to_load_retweets as f64 / 1_000_000.0f64);
            println!("  Time to process the retweets: {:.2}ms",
                     results.time_to_process_retweets as f64 / 1_000_000.0f64);
            println!("  Total time: {:.2}ms", results.total_time as f64 / 1_000_000.0f64);
            println!();
            println!("  Retweet Processing Rate: {} RT/s", results.retweet_processing_rate);
        },
        Err(error) => {
            println!("Error: {message}", message = error);
        }
    }
}
