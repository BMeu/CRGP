// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Filter possible influence edges.

use std::collections::HashMap;
use std::hash::Hash;

use timely::dataflow::Stream;
use timely::dataflow::Scope;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::unary::Unary;

use UserID;
use social_graph::InfluenceEdge;

/// Filter possible influence edges.
pub trait PrefixFilter<G: Scope> {
    /// Mark the influencee active and check if the possible influencer really has been active before the Retweeter. If
    /// so, pass on the influence edge.
    fn filter(&self) -> Stream<G, InfluenceEdge<UserID>>;
}

impl<G: Scope> PrefixFilter<G> for Stream<G, InfluenceEdge<UserID>>
where G::Timestamp: Hash {
    fn filter(&self) -> Stream<G, InfluenceEdge<UserID>> {
        // For each cascade, given by its ID, a set of activated users, given by their ID, i.e.
        // those users who have retweeted within this cascade before, per worker.
        let mut activations: HashMap<u64, HashMap<UserID, u64>> = HashMap::new();

        self.unary_stream(
            Exchange::new(|influence: &InfluenceEdge<UserID>| influence.influencer as u64),
            "PrefixFilter",
            move |influences, output| {
                // Process the influence edges: immediately pass them on and save them for batched writing.
                influences.for_each(|time, influence_data| {
                    let mut session = output.session(&time);
                    for influence in influence_data.drain(..) {
                        // Get the activations for this cascade.
                        let mut cascade_activations = activations.entry(influence.cascade_id)
                            .or_insert_with(HashMap::new);

                        // Mark the retweeting user active.
                        let _ = cascade_activations.insert(influence.influencee, influence.timestamp);

                        // Determine if the friend has been active before the Retweeter.
                        let is_influencer_activated: bool = match cascade_activations.get(&influence.influencer) {
                            Some(activation_timestamp) => &influence.timestamp > activation_timestamp,
                            None => false
                        };
                        let is_influencer_original_user: bool = influence.influencer == influence.original_user;

                        if is_influencer_activated || is_influencer_original_user {
                            session.give(influence);
                        }
                    }
                });
            }
        )
    }
}
