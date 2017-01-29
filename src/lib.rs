//! Reconstruct a retweet cascade.

#![warn(missing_docs)]

extern crate timely;

pub use hash::*;

mod hash;
pub mod social_graph;
pub mod timely_operators;

/// An edge in a graph.
pub type Edge<T> = (T, T);
