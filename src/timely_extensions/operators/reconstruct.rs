//! Reconstruct retweet cascades.

use std::collections::HashMap;
use std::hash::*;

use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::pact::{Exchange, Pipeline};
use timely::dataflow::operators::binary::Binary;

use social_graph::edge::*;
use twitter::Tweet;

/// Reconstruct retweet cascades.
pub trait Reconstruct<G: Scope> {
    /// Reconstruct retweet cascades, that is, find all influences edges within a social graph, distinguishing between
    /// cascades.
    ///
    /// For a social graph, determine all influences for a retweet within that specific retweet cascade. The ``Stream``
    /// of retweets may contain multiple retweet cascades. Each retweet in the retweet stream is expected to be
    /// broadcast to all workers before calling this operator.
    fn reconstruct(&self, graph: Stream<G, DirectedEdge<u64>>) -> Stream<G, InfluenceEdge<u64>>;
}

impl<G: Scope> Reconstruct<G> for Stream<G, Tweet>
where G::Timestamp: Hash {
    fn reconstruct(&self, graph: Stream<G, DirectedEdge<u64>>) -> Stream<G, InfluenceEdge<u64>> {
        // For each user, given by their ID, the set of their friends, given by their ID.
        let mut edges: HashMap<u64, Vec<u64>> = HashMap:: new();

        // For each cascade, given by its ID, a set of activated users, given by their ID, i.e. those users who have
        // retweeted within this cascade before, per worker. Users are associated with the time at which they first
        // retweeted within a cascade.
        let mut activations: HashMap<u64, HashMap<u64, u64>> = HashMap::new();

        self.binary_stream(
            &graph,
            Pipeline,
            Exchange::new(|edge: &DirectedEdge<u64>| edge.source),
            "Reconstruct",
            move |retweets, friendships, output| {
                // Input 1: Process the retweets.
                retweets.for_each(|time, retweet_data| {
                    for ref retweet in retweet_data.iter() {
                        // Tell the compiler the retweet is of type 'Tweet'.
                        let retweet: &Tweet = retweet;

                        // Skip all tweets that are not retweets.
                        let original_tweet: &Tweet = match retweet.retweeted_status {
                            Some(ref t) => t,
                            None => continue
                        };

                        // Mark this user as active for this cascade.
                        let ref mut cascade_activations: HashMap<u64, u64> = *activations.entry(original_tweet.id)
                            .or_insert(HashMap::new());
                        cascade_activations.entry(retweet.user.id)
                            .or_insert(retweet.created_at);

                        // If this is the worker storing the retweeting user's friends, find
                        // all influences. Otherwise, move on.
                        let friends: &Vec<u64> = match edges.get(&retweet.user.id) {
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
                                    Some(activation_timestamp) => &retweet.created_at >= activation_timestamp,
                                    None => false
                                };
                                let is_influencer_original_user: bool = friend == original_tweet.user.id;
                                if is_influencer_activated || is_influencer_original_user {
                                    let influence = InfluenceEdge::new(friend, retweet.user.id, retweet.created_at,
                                                                       original_tweet.id);
                                    output.session(&time).give(influence);
                                }
                            }
                        } else {
                            // Iterate over the activations.
                            for (user_id, activation_timestamp) in cascade_activations {
                                // If the current activation is not a friend, move on.
                                let friend: u64 = match friends.get(*user_id as usize) {
                                    Some(friend) => *friend,
                                    None => continue
                                };

                                // Ensure the influence is possible.
                                let is_influencer_activated: bool = &retweet.created_at >= activation_timestamp;
                                let is_influencer_original_user: bool = friend == original_tweet.user.id;
                                if is_influencer_activated || is_influencer_original_user {
                                    let influence = InfluenceEdge::new(friend, retweet.user.id, retweet.created_at,
                                                                       original_tweet.id);
                                    output.session(&time).give(influence);
                                }
                            }
                        }
                    };
                });

                // Input 2: Capture all friends for each user.
                friendships.for_each(|_time, friendship_data| {
                    for ref friendship in friendship_data.iter() {
                        edges.entry(friendship.source)
                            .or_insert(Vec::new())
                            .push(friendship.destination);
                    };
                });
            }
        )
    }
}
