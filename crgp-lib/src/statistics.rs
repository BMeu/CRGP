// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Collection of statistics about the execution of the algorithm.

/// Collection of statistics about the execution of the algorithm.
///
/// Times are given in nanoseconds.
#[derive(Clone, Copy, Debug)]
pub struct Statistics {
    /// Number of friendships in the social graph.
    number_of_friendships: u64,

    /// Number of retweets processed.
    number_of_retweets: u64,

    /// Size of the Retweet batches.
    batch_size: usize,

    /// Time to set up the computation.
    time_to_setup: u64,

    /// Time to load and process the social graph.
    time_to_process_social_graph: u64,

    /// Time to load the retweets.
    time_to_load_retweets: u64,

    /// Time to process the retweets.
    time_to_process_retweets: u64,

    /// Total time of the computation.
    total_time: u64
}

impl Statistics {
    /// Collect statistics about the influence computation. Times must be given in nanoseconds.
    pub fn new(number_of_friendships: u64, number_of_retweets: u64, batch_size: usize, time_to_setup: u64,
               time_to_process_social_graph: u64, time_to_load_retweets: u64, time_to_process_retweets: u64,
               total_time: u64) -> Statistics {
        Statistics { number_of_friendships: number_of_friendships, number_of_retweets: number_of_retweets,
            batch_size: batch_size, time_to_setup: time_to_setup,
            time_to_process_social_graph: time_to_process_social_graph, time_to_load_retweets: time_to_load_retweets,
            time_to_process_retweets: time_to_process_retweets, total_time: total_time }
    }

    /// Get the number of friendships in the social graph.
    pub fn number_of_friendships(&self) -> u64 {
        self.number_of_friendships
    }

    /// Get the total number of retweets processed.
    pub fn number_of_retweets(&self) -> u64 {
        self.number_of_retweets
    }

    /// Get the size of the Retweet batches.
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    /// Get the time to set up the computation (in nanoseconds).
    pub fn time_to_setup(&self) -> u64 {
        self.time_to_setup
    }

    /// Get the time to load and process the social graph (in nanoseconds).
    pub fn time_to_process_social_graph(&self) -> u64 {
        self.time_to_process_social_graph
    }

    /// Get the time to load the retweets (in nanoseconds).
    pub fn time_to_load_retweets(&self) -> u64 {
        self.time_to_load_retweets
    }

    /// Get the time to process the retweets (in nanoseconds).
    pub fn time_to_process_retweets(&self) -> u64 {
        self.time_to_process_retweets
    }

    /// Get the total time it took the computation to finish (in nanoseconds).
    pub fn total_time(&self) -> u64 {
        self.total_time
    }

    /// Get the average Retweet processing rate in Retweets per seconds (RT/s).
    pub fn retweet_processing_rate(&self) -> u64 {
        (self.number_of_retweets as f64 / (self.time_to_process_retweets as f64 / 1_000_000_000.0f64)) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.number_of_friendships, 1);
        assert_eq!(statistics.number_of_retweets, 2);
        assert_eq!(statistics.batch_size, 3);
        assert_eq!(statistics.time_to_setup, 4);
        assert_eq!(statistics.time_to_process_social_graph, 5);
        assert_eq!(statistics.time_to_load_retweets, 6);
        assert_eq!(statistics.time_to_process_retweets, 7);
        assert_eq!(statistics.total_time, 8);
    }

    #[test]
    fn number_of_friendships() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.number_of_friendships(), 1);
    }

    #[test]
    fn number_of_retweets() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.number_of_retweets(), 2);
    }

    #[test]
    fn batch_size() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.batch_size(), 3);
    }

    #[test]
    fn time_to_setup() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.time_to_setup(), 4);
    }

    #[test]
    fn time_to_process_social_graph() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.time_to_process_social_graph(), 5);
    }

    #[test]
    fn time_to_load_retweets() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.time_to_load_retweets(), 6);
    }

    #[test]
    fn time_to_process_retweets() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.time_to_process_retweets(), 7);
    }

    #[test]
    fn total_time() {
        let statistics = Statistics::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(statistics.total_time(), 8);
    }

    #[test]
    fn retweet_processing_rate() {
        let statistics = Statistics::new(0, 3, 0, 0, 0, 0, 2_000_000_000, 0);
        // 1.5 RT/s => 1 RT/s.
        assert_eq!(statistics.retweet_processing_rate(), 1);
    }
}
