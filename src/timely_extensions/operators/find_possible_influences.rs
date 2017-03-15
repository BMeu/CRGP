//! Find possible influence edges.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::*;
use std::rc::Rc;

use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::binary::Binary;

use social_graph::edge::*;
use twitter::Tweet;

/// Find possible influence edges within social graphs.
pub trait FindPossibleInfluences<G: Scope> {
    /// Find all possible influence edges within a social graph, distinguishing between cascades.
    ///
    /// For a social graph, determine all possible influences for a retweet within that specific
    /// retweet cascade. The ``Stream`` of retweets may contain multiple retweet cascades.
    fn find_possible_influences(&self, retweets: Stream<G, Tweet>,
                                activated_users: Rc<RefCell<HashMap<u64, HashMap<u64, u64>>>>)
        -> Stream<G, InfluenceEdge<u64>>;
}

impl<G: Scope> FindPossibleInfluences<G> for Stream<G, DirectedEdge<u64>>
where G::Timestamp: Hash {
    fn find_possible_influences(&self, retweets: Stream<G, Tweet>,
                                activated_users: Rc<RefCell<HashMap<u64, HashMap<u64, u64>>>>)
        -> Stream<G, InfluenceEdge<u64>> {
        // For each user, given by their ID, the set of their friends, given by their ID.
        let mut edges: HashMap<u64, HashSet<u64>> = HashMap::new();

        self.binary_stream(
            &retweets,
            Exchange::new(|edge: &DirectedEdge<u64>| edge.source),
            Exchange::new(|retweet: &Tweet| retweet.user.id),
            "FindPossibleInfluences",
            move |friendships, retweets, output| {
                // Input 1: Capture all friends for each user.
                friendships.for_each(|_time, friendship_data| {
                    for ref friends in friendship_data.iter() {
                        edges.entry(friends.source)
                            .or_insert(HashSet::new())
                            .insert(friends.destination);
                    }
                });

                // Input 2: Process the retweets.
                retweets.for_each(|time, retweet_data| {
                    for ref retweet in retweet_data.iter() {
                        // Skip all tweets that are not retweets.
                        let original_tweet: &Tweet = match retweet.retweeted_status {
                            Some(ref t) => t,
                            None => continue
                        };

                        // Mark this user and the original user as active for this cascade.
                        activated_users.borrow_mut()
                            .entry(original_tweet.id)
                            .or_insert(HashMap::new())
                            .entry(retweet.user.id)
                            .or_insert(retweet.created_at);

                        // Get the user's friends.
                        let friends = match edges.get(&retweet.user.id) {
                            Some(friends) => friends,
                            None => continue
                        };

                        // Pass on the possible influence edges.
                        for &friend in friends {
                            let influence = InfluenceEdge::new(friend, retweet.user.id, retweet.created_at, original_tweet.id, original_tweet.user.id);
                            output.session(&time).give(influence);
                        }
                    }
                });
            }
        )
    }
}
