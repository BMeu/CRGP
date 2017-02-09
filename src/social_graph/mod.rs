//! Traits for operating on a social graph.
//!
//! A social graph is a collection of directed edges.

pub use self::edge::DirectedEdge;
pub use self::edge::InfluenceEdge;

pub mod edge;
pub mod load;
