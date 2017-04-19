// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let edge: DirectedEdge<f64> = DirectedEdge::new(42.0, 13.37);
        assert_eq!(edge.source, 42.0);
        assert_eq!(edge.destination, 13.37);
    }
}
