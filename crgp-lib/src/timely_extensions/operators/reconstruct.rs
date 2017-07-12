// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Reconstruct retweet cascades.

use std::collections::HashMap;
use std::hash::Hash;

use timely::dataflow::Stream;
use timely::dataflow::Scope;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::binary::Binary;

use social_graph::InfluenceEdge;
use social_graph::SocialGraph;
use twitter::Retweet;
use twitter::Tweet;
use twitter::User;

/// Reconstruct retweet cascades.
pub trait Reconstruct<G: Scope> {
    /// Reconstruct retweet cascades, that is, find all influences edges within a social graph, distinguishing between
    /// cascades.
    ///
    /// For a social graph, determine all influences for a retweet within that specific retweet cascade. The `Stream`
    /// of retweets may contain multiple retweet cascades. Each retweet in the retweet stream is expected to be
    /// broadcast to all workers before calling this operator.
    fn reconstruct(&self, graph: Stream<G, (User, Vec<User>)>) -> Stream<G, InfluenceEdge<User>>;
}

impl<G: Scope> Reconstruct<G> for Stream<G, Retweet>
where G::Timestamp: Hash {
    fn reconstruct(&self, graph: Stream<G, (User, Vec<User>)>) -> Stream<G, InfluenceEdge<User>> {
        // For each user, given by their ID, the set of their friends, given by their ID.
        let mut edges = SocialGraph::new();

        // For each cascade, given by its ID, a set of activated users, given by their ID, i.e. those users who have
        // retweeted within this cascade before, per worker. Users are associated with the time at which they first
        // retweeted within a cascade.
        let mut activations: HashMap<u64, HashMap<User, u64>> = HashMap::new();

        self.binary_stream(
            &graph,
            Pipeline,
            Exchange::new(|friendships: &(User, Vec<User>)| friendships.0.id as u64),
            "Reconstruct",
            move |retweets, friendships, output| {
                // Input 1: Process the retweets.
                retweets.for_each(|time, retweet_data| {
                    let mut session = output.session(&time);
                    for retweet in retweet_data.take().iter() {
                        let original_tweet: &Tweet = &retweet.retweeted_status;

                        // Mark this user as active for this cascade.
                        let cascade_activations: &mut HashMap<User, u64> = &mut (*activations.entry(original_tweet.id)
                            .or_insert_with(|| {
                                // Create a new map for the activations of this cascade and insert the original tweeter.
                                let mut cascade_activations = HashMap::new();
                                let _ = cascade_activations.insert(original_tweet.user, original_tweet.created_at);
                                cascade_activations
                            }));
                        let _ = cascade_activations.entry(retweet.user)
                            .or_insert(retweet.created_at);

                        // If this is the worker storing the retweeting user's friends, find
                        // all influences. Otherwise, move on.
                        let friends: &Vec<User> = match edges.get(&retweet.user) {
                            Some(friends) => friends,
                            None => continue
                        };

                        // If the number of friends is smaller than the number of activations for
                        // this cascade, iterate over the friends, otherwise iterate over the
                        // activations.
                        if friends.len() <= cascade_activations.len() {
                            // Iterate over the friends.
                            for &friend in friends {
                                let is_influencer_activated: bool = match cascade_activations.get(&friend) {
                                    Some(activation_timestamp) => &retweet.created_at > activation_timestamp,
                                    None => false
                                };
                                if is_influencer_activated {
                                    let influence = InfluenceEdge::new(friend, retweet.user, retweet.created_at,
                                                                       retweet.id, original_tweet.id,
                                                                       original_tweet.user);
                                    session.give(influence);
                                }
                            }
                        } else {
                            // Iterate over the activations.
                            for (user, activation_timestamp) in cascade_activations {
                                // If the current activation is not a friend, move on.
                                let friend: User;
                                if let Ok(_index) = friends.binary_search(user) {
                                    friend = *user;
                                } else {
                                    continue;
                                }

                                // Ensure the influence is possible.
                                let is_influencer_activated: bool = &retweet.created_at > activation_timestamp;
                                if is_influencer_activated {
                                    let influence = InfluenceEdge::new(friend, retweet.user, retweet.created_at,
                                                                       retweet.id, original_tweet.id,
                                                                       original_tweet.user);
                                    session.give(influence);
                                }
                            }
                        }
                    };
                });

                // Input 2: Capture all friends for each user.
                friendships.for_each(|_time, friendship_data| {
                    for friendship in friendship_data.drain(..) {
                        let user: User = friendship.0;
                        let friends: Vec<User> = friendship.1;

                        let friendship_set: &mut Vec<User> = edges.entry(user)
                            .or_insert_with(|| Vec::with_capacity(friends.len()));
                        friendship_set.extend(friends);
                        friendship_set.shrink_to_fit();
                        friendship_set.sort();
                    };

                    edges.shrink_to_fit();
                });
            }
        )
    }
}
