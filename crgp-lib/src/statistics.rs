// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Collection of statistics about the execution of the algorithm.

use std::fmt;

use Configuration;

/// Collection of statistics about the execution of the algorithm.
///
/// Times are given in nanoseconds.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statistics {
    /// Number of friendships in the social graph.
    pub number_of_friendships: u64,

    /// Number of retweets processed.
    pub number_of_retweets: u64,

    /// Time to set up the computation (in `ns`).
    pub time_to_setup: u64,

    /// Time to load and process the social graph (in `ns`).
    pub time_to_process_social_graph: u64,

    /// Time to load the retweets (in `ns`).
    pub time_to_load_retweets: u64,

    /// Time to process the retweets (in `ns`).
    pub time_to_process_retweets: u64,

    /// Total time of the computation (in `ns`).
    pub total_time: u64,

    /// Average Retweet processing rate in Retweets per seconds (`RT/s`).
    ///
    /// This field will automatically be set whenever `number_of_retweets` or `time_to_process_retweets` are set.
    pub retweet_processing_rate: u64,

    /// The algorithm used for reconstruction.
    pub configuration: Configuration,

    /// Private field to prevent initialization without the provided methods.
    ///
    /// All other fields should be public for easy access without getter functions. However, adding more fields later
    /// could break code if the `Statistics` were manually initialized.
    #[serde(skip_serializing)]
    _prevent_outside_initialization: bool,
}

impl Statistics {
    /// Initialize default statistics.
    pub fn new(configuration: Configuration) -> Statistics {
        Statistics {
            configuration: configuration,
            number_of_friendships: 0,
            number_of_retweets: 0,
            time_to_setup: 0,
            time_to_process_social_graph: 0,
            time_to_load_retweets: 0,
            time_to_process_retweets: 0,
            total_time: 0,
            retweet_processing_rate: 0,
            _prevent_outside_initialization: true
        }
    }

    /// Set the number of friendships in the social graph.
    pub fn number_of_friendships(mut self, number_of_friendships: u64) -> Statistics {
        self.number_of_friendships = number_of_friendships;
        self
    }

    /// Set the total number of retweets processed.
    ///
    /// Also automatically sets the Retweet processing rate (if the Retweet processing rate is not `0`).
    pub fn number_of_retweets(mut self, number_of_retwets: u64) -> Statistics {
        self.number_of_retweets = number_of_retwets;
        if self.retweet_processing_rate != 0 {
            self.calculate_retweet_processing_rate();
        }
        self
    }

    /// Set the time to set up the computation (in nanoseconds).
    pub fn time_to_setup(mut self, setup_time: u64) -> Statistics {
        self.time_to_setup = setup_time;
        self
    }

    /// Set the time to load and process the social graph (in nanoseconds).
    pub fn time_to_process_social_graph(mut self, social_graph_processing_time: u64) -> Statistics {
        self.time_to_process_social_graph = social_graph_processing_time;
        self
    }

    /// Set the time to load the retweets (in nanoseconds).
    pub fn time_to_load_retweets(mut self, retweet_loading_time: u64) -> Statistics {
        self.time_to_load_retweets = retweet_loading_time;
        self
    }

    /// Set the time to process the retweets (in nanoseconds).
    ///
    /// Also automatically sets the Retweet processing rate.
    pub fn time_to_process_retweets(mut self, retweet_processing_time: u64) -> Statistics {
        self.time_to_process_retweets = retweet_processing_time;
        self.calculate_retweet_processing_rate();
        self
    }

    /// Set the total time it took the computation to finish (in nanoseconds).
    pub fn total_time(mut self, total_time: u64) -> Statistics {
        self.total_time = total_time;
        self
    }

    /// Set the average Retweet processing rate in Retweets per seconds (RT/s).
    ///
    /// If the time it took to process the retweets is 0, the rate will be set to 0 as well.
    fn calculate_retweet_processing_rate(&mut self) {
        self.retweet_processing_rate = if self.time_to_process_retweets == 0 {
            0
        } else {
            (self.number_of_retweets * 1_000_000_000) / self.time_to_process_retweets
        };
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
               "(Number of Friendships: {friendships}, Number of Retweets: {retweets}, Time to Set Up: {setup}ns, \
                Time to Process Social Graph: {graph}ns, Time to Load Retweets: {retweet_loading}ns, \
                Time to Process Retweets: {retweet_processing}ns, Total Time: {total}ns, \
                Retweet Processing Rate: {rate}RT/s, Configuration: {configuration})",
               friendships = self.number_of_friendships, retweets = self.number_of_retweets, setup = self.time_to_setup,
               graph = self.time_to_process_social_graph, retweet_loading = self.time_to_load_retweets,
               retweet_processing = self.time_to_process_retweets, total = self.total_time,
               rate = self.retweet_processing_rate, configuration = self.configuration)
    }
}

#[cfg(test)]
mod tests {

    use configuration::InputSource;
    use super::*;

    #[test]
    fn new() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone());
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 0);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn number_of_friendships() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone())
            .number_of_friendships(42);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 42);
        assert_eq!(statistics.number_of_retweets, 0);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn number_of_retweets() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let mut statistics = Statistics::new(configuration.clone())
            .number_of_retweets(42);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 42);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);

        statistics.retweet_processing_rate = 42;
        statistics.time_to_process_retweets = 42;
        let statistics = statistics.number_of_retweets(42);
        assert_eq!(statistics.number_of_retweets, 42);
        assert_eq!(statistics.retweet_processing_rate, 1_000_000_000);
    }

    #[test]
    fn time_to_setup() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone())
            .time_to_setup(42);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 0);
        assert_eq!(statistics.time_to_setup, 42);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn time_to_process_social_graph() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone())
            .time_to_process_social_graph(42);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 0);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 42);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn time_to_load_retweets() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone())
            .time_to_load_retweets(42);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 0);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 42);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn time_to_process_retweets() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        // The Retweet processing rate should also be updated (if number of Retweets is given).
        let statistics = Statistics::new(configuration.clone())
            .number_of_retweets(3)
            .time_to_process_retweets(2_000_000_000);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 3);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 2_000_000_000);
        assert_eq!(statistics.total_time, 0);
        assert_eq!(statistics.retweet_processing_rate, 1);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn total_time() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone())
            .total_time(42);
        assert_eq!(statistics.configuration, configuration);
        assert_eq!(statistics.number_of_friendships, 0);
        assert_eq!(statistics.number_of_retweets, 0);
        assert_eq!(statistics.time_to_setup, 0);
        assert_eq!(statistics.time_to_process_social_graph, 0);
        assert_eq!(statistics.time_to_load_retweets, 0);
        assert_eq!(statistics.time_to_process_retweets, 0);
        assert_eq!(statistics.total_time, 42);
        assert_eq!(statistics.retweet_processing_rate, 0);
        assert!(statistics._prevent_outside_initialization);
    }

    #[test]
    fn retweet_processing_rate() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let mut statistics = Statistics::new(configuration.clone());
        statistics.number_of_retweets = 3;
        statistics.time_to_process_retweets = 2_000_000_000;
        statistics.calculate_retweet_processing_rate();
        // 1.5 RT/s => 1 RT/s.
        assert_eq!(statistics.retweet_processing_rate, 1);
    }

    /// Old way of computing the Retweet processing rate.
    fn retweet_processing_rate_using_float(number_of_retweets: u64, time_to_process_retweets: u64) -> u64 {
        if time_to_process_retweets == 0 {
            return 0;
        }

        (number_of_retweets as f64 / (time_to_process_retweets as f64 / 1_000_000_000.0f64)) as u64
    }

    quickcheck! {
        /// The difference between the old and the new rate should not be greater than 1. A difference of 1 is within
        /// measurement inaccuracies.
        #[allow(trivial_casts)]
        fn compare_retweet_processing_rate_calcs(number_of_retweets: u64, time_to_process_retweets: u64) -> bool {
            let retweets = InputSource::new("path/to/retweets.json");
            let social_graph = InputSource::new("path/to/social/graph");
            let configuration = Configuration::default(retweets, social_graph);
            let statistics = Statistics::new(configuration)
                .number_of_retweets(number_of_retweets)
                .time_to_process_retweets(time_to_process_retweets);

            let old_rate = retweet_processing_rate_using_float(number_of_retweets, time_to_process_retweets);
            let new_rate = statistics.retweet_processing_rate;

            let difference = if new_rate > old_rate {
                new_rate - old_rate
            } else {
                old_rate - new_rate
            };
            difference <= 1
        }
    }

    #[test]
    fn fmt_display() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let configuration = Configuration::default(retweets, social_graph);

        let statistics = Statistics::new(configuration.clone());

        let fmt = "(Number of Friendships: 0, Number of Retweets: 0, Time to Set Up: 0ns, \
                   Time to Process Social Graph: 0ns, Time to Load Retweets: 0ns, Time to Process Retweets: 0ns, \
                   Total Time: 0ns, Retweet Processing Rate: 0RT/s, Configuration: \
                    (Algorithm: GALE, Batch Size: 50000, Hosts: [], Number of Processes: 1, \
                    Number of Workers: 1, Output Target: STDOUT, Insert Dummy Users: false, \
                    Process ID: 0, Report Connection Progress: false, Retweet Data Set: path/to/retweets.json, \
                    Social Graph: path/to/social/graph)\
                   )";
        assert_eq!(format!("{}", statistics), fmt);
    }
}
