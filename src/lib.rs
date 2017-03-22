//! The library for reconstructing Retweet cascades using a graph-parallel approach.

#![warn(missing_docs)]

#[macro_use]
extern crate abomonation;
extern crate fine_grained;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate timely;
extern crate timely_communication;

pub use error::{Error, Result};
pub use statistics::Statistics;

pub mod algorithm;
mod error;
pub mod social_graph;
mod statistics;
pub mod timely_extensions;
pub mod twitter;