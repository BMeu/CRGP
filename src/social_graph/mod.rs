//! Traits for operating on a social graph.
//!
//! A social graph is a collection of directed edges.

pub use self::directed_edge::DirectedEdge;
pub use self::influence_edge::InfluenceEdge;

pub mod directed_edge;
pub mod influence_edge;
pub mod source;
