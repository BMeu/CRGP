// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Algorithm configuration.

use timely_extensions::operators::OutputTarget;
use timely_communication::initialize::Configuration as TimelyConfiguration;

use Result;

/// Configuration for the `CRGP` algorithm.
///
/// # Example
///
/// The configuration can easily be built from default values:
///
/// ```rust
/// use std::path::PathBuf;
///
/// use crgp_lib::Configuration;
/// use crgp_lib::timely_extensions::operators::OutputTarget;
///
/// let retweets = String::from("path/to/retweets.json");
/// let social_graph = String::from("path/to/social/graph");
/// let output = PathBuf::from("results");
///
/// let configuration = Configuration::default(retweets, social_graph)
///     .output_target(OutputTarget::Directory(output))
///     .workers(2);
///
/// assert_eq!(configuration.batch_size, 500);
/// assert_eq!(configuration.hosts, None);
/// assert_eq!(configuration.number_of_processes, 1);
/// assert_eq!(configuration.number_of_workers, 2);
/// assert_eq!(configuration.output_target,
///            OutputTarget::Directory(PathBuf::from("results")));
/// assert_eq!(configuration.process_id, 0);
/// assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
/// assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
/// ```
#[derive(Clone, Debug)]
pub struct Configuration {
    /// Number of Retweets being processed at once.
    pub batch_size: usize,

    /// Path to the file containing the host list.
    pub hosts: Option<String>,

    /// Number of processes involved in the computation.
    pub number_of_processes: u64,

    /// Number of per-process worker threads.
    pub number_of_workers: u64,

    /// Target for writing results.
    pub output_target: OutputTarget,

    /// Identity of this process, from `0` to `number_of_processes - 1`.
    pub process_id: u64,

    /// Path to the file containing the Retweets.
    pub retweets: String,

    /// Path to the data set containing the social graph.
    pub social_graph: String,
}

impl Configuration {
    /// Initialize a configuration with default values.
    ///
    /// The following default values will be set:
    ///
    ///  * `batch_size`: `500`
    ///  * `hosts`: `None`
    ///  * `number_of_processes`: `1`
    ///  * `number_of_workers`: `1`
    ///  * `output_target`: `OutputTarget::StdOut`
    ///  * `process_id`: `0`
    pub fn default(retweets: String, social_graph: String) -> Configuration {
        Configuration {
            batch_size: 500,
            hosts: None,
            number_of_processes: 1,
            number_of_workers: 1,
            output_target: OutputTarget::StdOut,
            process_id: 0,
            retweets: retweets,
            social_graph: social_graph,
        }
    }

    /// Set the batch size.
    #[inline]
    pub fn batch_size(mut self, batch_size: usize) -> Configuration {
        self.batch_size = batch_size;
        self
    }

    /// Set the path to the host list file.
    #[inline]
    pub fn hosts(mut self, hosts: Option<String>) -> Configuration {
        self.hosts = hosts;
        self
    }

    /// Set the target for writing results.
    #[inline]
    pub fn output_target(mut self, target: OutputTarget) -> Configuration {
        self.output_target = target;
        self
    }

    /// Set the identity of this process.
    #[inline]
    pub fn process_id(mut self, id: u64) -> Configuration {
        self.process_id = id;
        self
    }

    /// Set the number of involved processes.
    #[inline]
    pub fn processes(mut self, processes: u64) -> Configuration {
        self.number_of_processes = processes;
        self
    }

    /// Set the number of per-process workers.
    #[inline]
    pub fn workers(mut self, workers: u64) -> Configuration {
        self.number_of_workers = workers;
        self
    }

    /// Determine the configuration for `timely`.
    ///
    /// This function mimics `timely_communication::initialize::Configuration::from_args()`.
    #[doc(hidden)]
    #[inline]
    pub fn get_timely_configuration(&self) -> Result<TimelyConfiguration> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use timely_extensions::operators::OutputTarget;

    use super::*;

    #[test]
    fn default() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph);

        assert_eq!(configuration.batch_size, 500);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn batch_size() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .batch_size(1);

        assert_eq!(configuration.batch_size, 1);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn hosts() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");
        let hosts = String::from("hosts.txt");

        let configuration = Configuration::default(retweets, social_graph)
            .hosts(Some(hosts));

        assert_eq!(configuration.batch_size, 500);
        assert_eq!(configuration.hosts, Some(String::from("hosts.txt")));
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn output_target() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");
        let output = PathBuf::from("results");

        let configuration = Configuration::default(retweets, social_graph)
            .output_target(OutputTarget::Directory(output));

        assert_eq!(configuration.batch_size, 500);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target,
                   OutputTarget::Directory(PathBuf::from("results")));
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn process_id() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .process_id(42);

        assert_eq!(configuration.batch_size, 500);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.process_id, 42);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn processes() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .processes(42);

        assert_eq!(configuration.batch_size, 500);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 42);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn workers() {
        let retweets = String::from("path/to/retweets.json");
        let social_graph = String::from("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .workers(42);

        assert_eq!(configuration.batch_size, 500);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 42);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.retweets, String::from("path/to/retweets.json"));
        assert_eq!(configuration.social_graph, String::from("path/to/social/graph"));
    }

    #[test]
    fn get_timely_configuration() {
        assert!(true);
    }
}
