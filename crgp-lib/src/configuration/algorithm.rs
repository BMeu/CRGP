// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Configuration for which algorithm to use.

use std::fmt;

/// Available algorithms for reconstruction.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Algorithm {
    /// Activate retweeting users on all workers, produce influence edges on the worker storing the user's friends.
    ///
    /// `GALE` = Global Activations, Local Edges
    GALE,

    /// Activate user and produce possible influences on worker storing the user's friends, filter possible influences
    /// on worker storing influencer's friends.
    ///
    /// `LEAF` = Local Edges, Activations, and Filtering
    LEAF,

    /// Not a reconstruction algorithm, but a computation to measure the throughput of messages.
    THROUGHPUT,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let algorithm_name: &str = match *self {
            Algorithm::GALE => "GALE",
            Algorithm::LEAF => "LEAF",
            Algorithm::THROUGHPUT => "Throughput",
        };
        write!(formatter, "{algorithm}", algorithm = algorithm_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_display_gale() {
        let algorithm = Algorithm::GALE;
        assert_eq!(format!("{}", algorithm), String::from("GALE"));
    }

    #[test]
    fn fmt_display_leaf() {
        let algorithm = Algorithm::LEAF;
        assert_eq!(format!("{}", algorithm), String::from("LEAF"));
    }

    #[test]
    fn fmt_display_throughput() {
        let algorithm = Algorithm::THROUGHPUT;
        assert_eq!(format!("{}", algorithm), String::from("Throughput"));
    }
}
