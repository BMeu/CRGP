//! A directed edge.

use abomonation::Abomonation;

/// A directed edge between nodes of type ``T``.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DirectedEdge<T>
where T: Abomonation {
    /// The source node.
    pub source: T,

    /// The destination node.
    pub destination: T
}

impl<T> DirectedEdge<T>
where T: Abomonation {
    /// Construct a new directed edge from ``source`` to ``destination``.
    pub fn new(source: T, destination: T) -> DirectedEdge<T> {
        DirectedEdge { source: source, destination: destination }
    }
}

unsafe_abomonate!(DirectedEdge<u64> : source, destination);
