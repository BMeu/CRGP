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

#[macro_use]
extern crate abomonation;
extern crate fine_grained;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate regex;
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
pub use execution::run;
pub use statistics::Statistics;
pub use twitter::UserID;

pub mod configuration;
pub mod execution;
mod error;
pub mod social_graph;
mod statistics;
pub mod timely_extensions;
pub mod twitter;