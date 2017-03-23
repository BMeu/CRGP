//! A directed edge representing influence in the social graph.

use abomonation::Abomonation;

/// A directed edge between nodes of type ``T`` representing influence in a Retweet cascade.
///
/// The influence flows from the ``influencer`` to the ``influencee`` and is valid only for the cascade given by
/// ``cascade_id``. The influence occurs at time ``timestamp``.
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
    /// Construct a new influence edge from ``influencer`` to ``influencee`` for the cascade ``cascade_id``, where the
    /// ``influencee`` was influenced at time ``timestamp``.
    pub fn new(influencer: T, influencee: T, timestamp: u64, cascade_id: u64) -> InfluenceEdge<T> {
        InfluenceEdge { influencer: influencer, influencee: influencee, timestamp: timestamp,
            cascade_id: cascade_id }
    }
}

unsafe_abomonate!(InfluenceEdge<u64> : influencer, influencee, timestamp, cascade_id);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let edge: InfluenceEdge<f64> = InfluenceEdge::new(42.0, 13.37, 12345, 67890);
        assert_eq!(edge.influencer, 42.0);
        assert_eq!(edge.influencee, 13.37);
        assert_eq!(edge.timestamp, 12345);
        assert_eq!(edge.cascade_id, 67890);
    }
}
