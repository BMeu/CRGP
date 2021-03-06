// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The main configuration object.

use std::fmt;
use std::path::PathBuf;

use timely_communication::initialize::Configuration as TimelyConfiguration;

use Error;
use Result;
use configuration::Algorithm;
use configuration::InputSource;
use configuration::OutputTarget;

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
/// use crgp_lib::configuration::Algorithm;
/// use crgp_lib::configuration::InputSource;
/// use crgp_lib::configuration::OutputTarget;
///
/// let retweets = InputSource::new("path/to/retweets.json");
/// let social_graph = InputSource::new("path/to/social/graph");
/// let output = PathBuf::from("results");
///
/// let configuration = Configuration::default(retweets, social_graph)
///     .output_target(OutputTarget::Directory(output))
///     .pad_with_dummy_users(true)
///     .workers(2);
///
/// assert_eq!(configuration.algorithm, Algorithm::GALE);
/// assert_eq!(configuration.batch_size, 50000);
/// assert_eq!(configuration.hosts, None);
/// assert_eq!(configuration.number_of_processes, 1);
/// assert_eq!(configuration.number_of_workers, 2);
/// assert_eq!(configuration.output_target,
///            OutputTarget::Directory(PathBuf::from("results")));
/// assert_eq!(configuration.pad_with_dummy_users, true);
/// assert_eq!(configuration.process_id, 0);
/// assert_eq!(configuration.report_connection_progress, false);
/// assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
/// assert_eq!(configuration.selected_users, None);
/// assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration {
    /// The algorithm used for reconstruction.
    pub algorithm: Algorithm,

    /// Number of Retweets being processed at once.
    pub batch_size: usize,

    /// A list of host addresses, each in the form `address:port`, where address may be a hostname or an IPv4 address.
    pub hosts: Option<Vec<String>>,

    /// Number of processes involved in the computation.
    pub number_of_processes: usize,

    /// Number of per-process worker threads.
    pub number_of_workers: usize,

    /// Target for writing results.
    #[serde(skip_serializing)]
    pub output_target: OutputTarget,

    /// If the given friend list for each user is only a subset of their friends, create as many dummy users as needed
    /// to reach the user's actual number of friends.
    ///
    /// This is useful if the social graph passed to `CRGP` contains only the friends that are known to be active in
    /// a given cascade (e.g. to save memory on disk), but you are interested in the real-world performance of `CRGP`.
    pub pad_with_dummy_users: bool,

    /// Identity of this process, from `0` to `number_of_processes - 1`.
    pub process_id: usize,

    /// Print connection progress to STDOUT when using multiple processes.
    pub report_connection_progress: bool,

    /// Path to the file containing the Retweets.
    pub retweets: InputSource,

    /// Path to a file containing the user IDs (one per line) that will be loaded from the social graph. Other users in
    /// the graph will be skipped. If `None`, all users will be loaded.
    pub selected_users: Option<PathBuf>,

    /// Path to the data set containing the social graph.
    pub social_graph: InputSource,

    /// Private field to prevent initialization without the provided methods.
    ///
    /// All other fields should be public for easy access without getter functions. However, adding more fields later
    /// could break code if the `Configuration` were manually initialized.
    #[serde(skip_serializing)]
    _prevent_outside_initialization: bool,
}

impl Configuration {
    /// Initialize a configuration with default values.
    ///
    /// The following default values will be set:
    ///
    ///  * `algorithm`: `Algorithm::GALE`
    ///  * `batch_size`: `50000`
    ///  * `hosts`: `None`
    ///  * `number_of_processes`: `1`
    ///  * `number_of_workers`: `1`
    ///  * `output_target`: `OutputTarget::StdOut`
    ///  * `pad_with_dummy_users`: `false`
    ///  * `process_id`: `0`
    ///  * `report_connection_progress`: `false`
    ///  * `selected_users`: `None`
    pub fn default(retweets: InputSource, social_graph: InputSource) -> Configuration {
        Configuration {
            algorithm: Algorithm::GALE,
            batch_size: 50000,
            hosts: None,
            number_of_processes: 1,
            number_of_workers: 1,
            output_target: OutputTarget::StdOut,
            pad_with_dummy_users: false,
            process_id: 0,
            report_connection_progress: false,
            retweets: retweets,
            selected_users: None,
            social_graph: social_graph,
            _prevent_outside_initialization: true,
        }
    }

    /// Choose the algorithm.
    #[inline]
    pub fn algorithm(mut self, algorithm: Algorithm) -> Configuration {
        self.algorithm = algorithm;
        self
    }

    /// Set the batch size.
    #[inline]
    pub fn batch_size(mut self, batch_size: usize) -> Configuration {
        self.batch_size = batch_size;
        self
    }

    /// Set the host list.
    #[inline]
    pub fn hosts(mut self, hosts: Option<Vec<String>>) -> Configuration {
        self.hosts = hosts;
        self
    }

    /// Set the target for writing results.
    #[inline]
    pub fn output_target(mut self, target: OutputTarget) -> Configuration {
        self.output_target = target;
        self
    }

    /// Toggle the creation of dummy users.
    #[inline]
    pub fn pad_with_dummy_users(mut self, pad: bool) -> Configuration {
        self.pad_with_dummy_users = pad;
        self
    }

    /// Set the identity of this process.
    #[inline]
    pub fn process_id(mut self, id: usize) -> Configuration {
        self.process_id = id;
        self
    }

    /// Set the number of involved processes.
    #[inline]
    pub fn processes(mut self, processes: usize) -> Configuration {
        self.number_of_processes = processes;
        self
    }

    /// Toggle connection progress reports.
    #[inline]
    pub fn report_connection_progress(mut self, report: bool) -> Configuration {
        self.report_connection_progress = report;
        self
    }

    /// Set the path to a file containing the user IDs (one per line) that will be loaded from the social graph. Other
    /// users in the graph will be skipped. If `None`, all users will be loaded.
    #[inline]
    pub fn selected_users(mut self, users: Option<PathBuf>) -> Configuration {
        self.selected_users = users;
        self
    }

    /// Set the number of per-process workers.
    #[inline]
    pub fn workers(mut self, workers: usize) -> Configuration {
        self.number_of_workers = workers;
        self
    }

    /// Determine the configuration for `timely`.
    ///
    /// This function mimics `timely_communication::initialize::Configuration::from_args()`.
    #[doc(hidden)]
    #[inline]
    pub fn get_timely_configuration(&mut self) -> Result<TimelyConfiguration> {
        if self.process_id >= self.number_of_processes {
            return Err(Error::from(String::from("the process ID is not in range of all processes")));
        }

        if self.number_of_processes > 1 {
            // Cluster of processes.

            // If no hosts are given, run on localhost.
            let mut host_addresses = Vec::<String>::new();
            if let Some(ref hosts) = self.hosts {
                if hosts.len() != self.number_of_processes {
                    return Err(Error::from(String::from(format!("{hosts} hosts given, but expected {processes}",
                                                                hosts = hosts.len(),
                                                                processes = self.number_of_processes))));
                }
                host_addresses = hosts.clone();
            } else {
                for index in 0..self.number_of_processes {
                    host_addresses.push(format!("localhost:{port}", port = 2101 + index));
                }

                self.hosts = Some(host_addresses.clone());
            }

            Ok(TimelyConfiguration::Cluster(self.number_of_workers, self.process_id, host_addresses,
                                            self.report_connection_progress))
        } else if self.number_of_workers > 1 {
            // One process, multiple workers.
            Ok(TimelyConfiguration::Process(self.number_of_workers))
        } else {
            // Single process, single thread.
            Ok(TimelyConfiguration::Thread)
        }
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let hosts: String = match self.hosts {
            Some(ref hosts) => {
                let mut hosts_list = String::from("[");
                let mut joined_hosts: String = hosts
                    .iter()
                    .fold(String::new(), |acc, s| {
                        acc + s + ", "
                    });
                let _ = joined_hosts.pop();
                let _ = joined_hosts.pop();
                hosts_list += &joined_hosts;
                hosts_list += "]";
                hosts_list
            }
            None => String::from("[]")
        };

        write!(formatter,
               "(Algorithm: {algorithm}, Batch Size: {batch}, Hosts: {hosts}, Number of Processes: {processes}, \
                Number of Workers: {workers}, Output Target: {output}, Insert Dummy Users: {dummies}, \
                Process ID: {id}, Report Connection Progress: {progress}, Retweet Data Set: {retweets}, \
                Social Graph: {graph})",
               algorithm = self.algorithm, batch = self.batch_size, hosts = hosts,
               processes = self.number_of_processes, workers = self.number_of_workers, output = self.output_target,
               dummies = self.pad_with_dummy_users, id = self.process_id, progress = self.report_connection_progress,
               retweets = self.retweets, graph = self.social_graph)
    }
}

#[cfg(test)]
mod tests {
    use configuration::Algorithm;
    use configuration::OutputTarget;
    use std::error::Error;
    use std::path::PathBuf;
    use timely_communication::initialize::Configuration as TimelyConfiguration;

    use super::*;

    #[test]
    fn default() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn algorithm() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .algorithm(Algorithm::LEAF);

        assert_eq!(configuration.algorithm, Algorithm::LEAF);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn batch_size() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .batch_size(1);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 1);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn hosts() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let hosts = vec![
            String::from("host1:2101"),
            String::from("host1:2102"),
            String::from("host1:2103"),
        ];

        let configuration = Configuration::default(retweets, social_graph)
            .hosts(Some(hosts));

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, Some(vec![
            String::from("host1:2101"),
            String::from("host1:2102"),
            String::from("host1:2103"),
        ]));
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn output_target() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");
        let output = PathBuf::from("results");

        let configuration = Configuration::default(retweets, social_graph)
            .output_target(OutputTarget::Directory(output));

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target,
        OutputTarget::Directory(PathBuf::from("results")));
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn pad_with_dummy_users() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .pad_with_dummy_users(true);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, true);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn process_id() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .process_id(42);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 42);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn processes() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .processes(42);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 42);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn report_connection_progress() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .report_connection_progress(true);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, true);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn selected_users() {
        let retweets = InputSource::new("path/to/retweets.json");
        let selected_users = PathBuf::from("path/to/selected/users.txt");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .selected_users(Some(selected_users));

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 1);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, Some(PathBuf::from("path/to/selected/users.txt")));
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn workers() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph)
            .workers(42);

        assert_eq!(configuration.algorithm, Algorithm::GALE);
        assert_eq!(configuration.batch_size, 50000);
        assert_eq!(configuration.hosts, None);
        assert_eq!(configuration.number_of_processes, 1);
        assert_eq!(configuration.number_of_workers, 42);
        assert_eq!(configuration.output_target, OutputTarget::StdOut);
        assert_eq!(configuration.pad_with_dummy_users, false);
        assert_eq!(configuration.process_id, 0);
        assert_eq!(configuration.report_connection_progress, false);
        assert_eq!(configuration.retweets, InputSource::new("path/to/retweets.json"));
        assert_eq!(configuration.selected_users, None);
        assert_eq!(configuration.social_graph, InputSource::new("path/to/social/graph"));
        assert!(configuration._prevent_outside_initialization);
    }

    #[test]
    fn get_timely_configuration() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        // Single thread by default.
        let mut configuration = Configuration::default(retweets.clone(), social_graph.clone());
        let timely_config = configuration.get_timely_configuration();
        assert!(timely_config.is_ok());
        match timely_config.expect("Failed to get the Timely configuration") {
            TimelyConfiguration::Thread => {
                assert!(true)
            },
            _ => assert!(false, "wrong timely configuration, expected `TimelyConfiguration::Thread`")
        }

        // Multiple threads.
        let mut configuration = Configuration::default(retweets.clone(), social_graph.clone())
            .workers(13);
        let timely_config = configuration.get_timely_configuration();
        assert!(timely_config.is_ok());
        match timely_config.expect("Failed to get the Timely configuration") {
            TimelyConfiguration::Process(workers) => {
                assert_eq!(workers, 13);
            },
            _ => assert!(false, "wrong timely configuration, expected `TimelyConfiguration::Process(..)`")
        }

        // Multiple processes, wrong process ID.
        let mut configuration = Configuration::default(retweets.clone(), social_graph.clone())
            .workers(13)
            .processes(42)
            .process_id(43);
        let timely_config = configuration.get_timely_configuration();
        assert!(timely_config.is_err());
        // Since `TimelyConfiguration` does not implement `Debug`, we have to get rid of it before calling `expect_err`.
        assert_eq!(timely_config.map(|_| ())
            .expect_err("unexpectedly succeeded getting the Timely configuration")
            .description(),
        "the process ID is not in range of all processes");

        // Multiple processes, with hosts, wrong number of addresses.
        let mut configuration = Configuration::default(retweets.clone(), social_graph.clone())
            .workers(13)
            .processes(42)
            .process_id(2)
            .hosts(Some(vec![
                String::from("host1:2101"),
                String::from("host1:2102"),
                String::from("host1:2103")
            ]));
        let timely_config = configuration.get_timely_configuration();
        assert!(timely_config.is_err());
        // Since `TimelyConfiguration` does not implement `Debug`, we have to get rid of it before calling `expect_err`.
        assert_eq!(timely_config.map(|_| ())
            .expect_err("unexpectedly succeeded getting the Timely configuration")
            .description(),
        "3 hosts given, but expected 42");

        // Multiple processes, with hosts.
        let mut configuration = Configuration::default(retweets.clone(), social_graph.clone())
            .workers(13)
            .processes(3)
            .process_id(2)
            .hosts(Some(vec![
                String::from("host1:2101"),
                String::from("host1:2102"),
                String::from("host1:2103")
            ]));
        let timely_config = configuration.get_timely_configuration();
        assert!(timely_config.is_ok());
        match timely_config.expect("Failed to get the Timely configuration") {
            TimelyConfiguration::Cluster(workers, id, hosts, report) => {
                assert_eq!(workers, 13);
                assert_eq!(id, 2);
                assert_eq!(hosts, vec![
                    String::from("host1:2101"),
                    String::from("host1:2102"),
                    String::from("host1:2103")
                ]);
                assert_eq!(report, false);
            },
            _ => assert!(false, "wrong timely configuration, expected `TimelyConfiguration::Cluster(..)`")
        }
        // The configuration must still contain the host list.
        assert_eq!(configuration.hosts, Some(vec![
            String::from("host1:2101"),
            String::from("host1:2102"),
            String::from("host1:2103")
        ]));

        // Multiple processes, without hosts.
        let mut configuration = Configuration::default(retweets.clone(), social_graph.clone())
            .workers(13)
            .processes(3)
            .process_id(2);
        let timely_config = configuration.get_timely_configuration();
        assert!(timely_config.is_ok());
        match timely_config.expect("Failed to get the Timely configuration") {
            TimelyConfiguration::Cluster(workers, id, hosts, report) => {
                assert_eq!(workers, 13);
                assert_eq!(id, 2);
                assert_eq!(hosts, vec![
                    String::from("localhost:2101"),
                    String::from("localhost:2102"),
                    String::from("localhost:2103")
                ]);
                assert_eq!(report, false);
            },
            _ => assert!(false, "wrong timely configuration, expected `TimelyConfiguration::Cluster(..)`")
        }
        // The config hosts should be set afterwards.
        assert_eq!(configuration.hosts, Some(vec![
            String::from("localhost:2101"),
            String::from("localhost:2102"),
            String::from("localhost:2103")
        ]));
    }

    #[test]
    fn fmt_display() {
        let retweets = InputSource::new("path/to/retweets.json");
        let social_graph = InputSource::new("path/to/social/graph");

        let configuration = Configuration::default(retweets, social_graph);

        let fmt = "(Algorithm: GALE, Batch Size: 50000, Hosts: [], Number of Processes: 1, \
                   Number of Workers: 1, Output Target: STDOUT, Insert Dummy Users: false, \
                   Process ID: 0, Report Connection Progress: false, Retweet Data Set: path/to/retweets.json, \
                   Social Graph: path/to/social/graph)";
        assert_eq!(format!("{}", configuration), String::from(fmt));

        let configuration = configuration.hosts(Some(vec![String::from("host1:port1"), String::from("host2:port2")]));

        let fmt = "(Algorithm: GALE, Batch Size: 50000, Hosts: [host1:port1, host2:port2], Number of Processes: 1, \
                   Number of Workers: 1, Output Target: STDOUT, Insert Dummy Users: false, \
                   Process ID: 0, Report Connection Progress: false, Retweet Data Set: path/to/retweets.json, \
                   Social Graph: path/to/social/graph)";
        assert_eq!(format!("{}", configuration), String::from(fmt));
    }
}
