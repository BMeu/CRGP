// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The library for reconstructing Retweet cascades using a graph-parallel approach.

#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]
#![cfg_attr(feature = "cargo-clippy", warn(empty_enum, enum_glob_use, if_not_else, items_after_statements,
                                           missing_docs_in_private_items, nonminimal_bool, option_unwrap_used,
                                           pub_enum_variant_names, print_stdout, result_unwrap_used, similar_names,
                                           single_match_else, stutter, used_underscore_binding, use_debug,
                                           wrong_self_convention, wrong_pub_self_convention))]

#[macro_use]
extern crate abomonation;
extern crate fine_grained;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate regex;
extern crate s3;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tar;
extern crate timely;
extern crate timely_communication;

pub use configuration::Algorithm;
pub use configuration::Configuration;
pub use configuration::OutputTarget;
pub use error::Error;
pub use error::Result;
pub use reconstruction::run;
pub use statistics::Statistics;
use twitter::UserID;

pub mod configuration;
mod error;
mod reconstruction;
mod social_graph;
mod statistics;
mod timely_extensions;
mod twitter;
