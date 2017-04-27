// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! A directed edge representing influence in the social graph.

use abomonation::Abomonation;

use UserID;

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

    /// The ID of the Retweet.
    pub retweet_id: u64,

    /// The ID of the Retweet cascade for which this influence is valid.
    pub cascade_id: u64,
}

impl<T> InfluenceEdge<T>
    where T: Abomonation {
    /// Construct a new influence edge from ``influencer`` to ``influencee`` for the cascade ``cascade_id``, where the
    /// ``influencee`` was influenced at time ``timestamp``.
    pub fn new(influencer: T, influencee: T, timestamp: u64, retweet_id: u64, cascade_id: u64) -> InfluenceEdge<T> {
        InfluenceEdge {
            influencer: influencer,
            influencee: influencee,
            timestamp: timestamp,
            retweet_id: retweet_id,
            cascade_id: cascade_id,
        }
    }
}

unsafe_abomonate!(InfluenceEdge<UserID> : influencer, influencee, timestamp, cascade_id);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let edge: InfluenceEdge<f64> = InfluenceEdge::new(42.0, 13.37, 123, 456, 789);
        assert_eq!(edge.influencer, 42.0);
        assert_eq!(edge.influencee, 13.37);
        assert_eq!(edge.timestamp, 123);
        assert_eq!(edge.retweet_id, 456);
        assert_eq!(edge.cascade_id, 789);
    }
}
