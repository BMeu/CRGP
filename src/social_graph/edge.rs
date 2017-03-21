//! Data structures for edges in a social graph.

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

/// A directed edge between nodes of type ``T`` representing influence in a Retweet cascade.
///
/// The influence flows from the ``influencer`` to the ``influencee`` and is valid only for the
/// cascade given by ``cascade_id``. The influence occurs at time ``timestamp``.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InfluenceEdge<T>
where T: Abomonation {
    /// The user influencing some other user.
    ///
    /// The source node of the directed edge.
    pub influencer: T,

    /// The user being influenced.
    ///
    /// The destination node of the directed edge.
    pub influencee: T,

    /// The time at which this influence is established.
    pub timestamp: u64,

    /// The ID of the Retweet cascade for which this influence is valid.
    pub cascade_id: u64
}

impl<T> InfluenceEdge<T>
where T: Abomonation {
    /// Construct a new influence edge from ``influencer`` to ``influencee`` for the cascade
    /// ``cascade_id``, where the ``influencee`` was influenced at time ``timestamp``.
    pub fn new(influencer: T, influencee: T, timestamp: u64, cascade_id: u64) -> InfluenceEdge<T> {
        InfluenceEdge { influencer: influencer, influencee: influencee, timestamp: timestamp,
                        cascade_id: cascade_id }
    }
}

unsafe_abomonate!(InfluenceEdge<u64> : influencer, influencee, timestamp, cascade_id);
