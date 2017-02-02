//! Reconstruct a retweet cascade.

#![warn(missing_docs)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate timely;

pub use hash::*;

mod hash;
pub mod social_graph;
pub mod timely_operators;
pub mod twitter;

/// An edge between two nodes of type ``T`` in a graph.
pub type Edge<T> = (T, T);
