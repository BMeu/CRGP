//! Reconstruct retweet cascades.

use std::collections::{HashMap, HashSet};
use std::hash::*;

use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::pact::{Exchange, Pipeline};
use timely::dataflow::operators::binary::Binary;

use social_graph::edge::*;
use twitter::Tweet;

/// Reconstruct retweet cascades.
pub trait Reconstruct<G: Scope> {
    /// Reconstruct retweet cascades, that is, find all influences edges within a social graph,
    /// distinguishing between cascades.
    ///
    /// For a social graph, determine all influences for a retweet within that specific retweet
    /// cascade. The ``Stream`` of retweets may contain multiple retweet cascades. Each retweet in
    /// the retweet stream is expected to be broadcast to all workers before calling this operator.
    fn reconstruct(&self, graph: Stream<G, DirectedEdge<u64>>) -> Stream<G, InfluenceEdge<u64>>;
}

impl<G: Scope> Reconstruct<G> for Stream<G, Tweet>
where G::Timestamp: Hash {
    fn reconstruct(&self, graph: Stream<G, DirectedEdge<u64>>) -> Stream<G, InfluenceEdge<u64>> {
        // For each user, given by their ID, the set of their friends, given by their ID.
        let mut edges: HashMap<u64, HashSet<u64>> = HashMap:: new();

        // For each cascade, given by its ID, a set of activated users, given by their ID, i.e.
        // those users who have retweeted within this cascade before, per worker.
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
                        let retweet: &Tweet = retweet;

                        // Skip all tweets that are not retweets.
                        let original_tweet: &Tweet = match retweet.retweeted_status {
                            Some(ref t) => t,
                            None => continue
                        };

                        // Mark this user as active for this cascade.
                        activations.entry(original_tweet.id)
                            .or_insert(HashMap::new())
                            .entry(retweet.user.id)
                            .or_insert(retweet.created_at);

                        // If this is the worker storing the retweeting user's friends, find
                        // all influences.
                        let friends = match edges.get(&retweet.user.id) {
                            Some(friends) => friends,
                            None => continue
                        };

                        for &friend in friends {
                            // Only send the influence if the user has been activated before.
                            let is_influencer_activated: bool = match activations.get(&original_tweet.id) {
                                Some(users) => match users.get(&friend) {
                                    Some(activation_timestamp) => &retweet.created_at >= activation_timestamp,
                                    None => false
                                },
                                None => false
                            };
                            let is_influencer_original_user: bool = friend == original_tweet.user.id;
                            if !(is_influencer_original_user || is_influencer_activated) {
                                continue;
                            };

                            let influence = InfluenceEdge::new(friend, retweet.user.id, retweet.created_at, original_tweet.id, original_tweet.user.id);
                            output.session(&time).give(influence);
                        };
                    };
                });

                // Input 2: Capture all friends for each user.
                friendships.for_each(|_time, friendship_data| {
                    for ref friends in friendship_data.iter() {
                        edges.entry(friends.source)
                            .or_insert(HashSet::new())
                            .insert(friends.destination);
                    };
                });
            }
        )
    }
}
