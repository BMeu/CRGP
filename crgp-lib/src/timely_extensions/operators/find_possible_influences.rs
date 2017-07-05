// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Find possible influence edges.

use std::hash::*;

use timely::dataflow::Scope;
use timely::dataflow::Stream;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::binary::Binary;

use UserID;
use social_graph::InfluenceEdge;
use social_graph::SocialGraph;
use twitter::Tweet;

/// Find possible influence edges within social graphs.
pub trait FindPossibleInfluences<G: Scope> {
    /// Find all possible influence edges within a social graph, distinguishing between cascades.
    ///
    /// For a social graph, determine all possible influences for a retweet within that specific
    /// retweet cascade. The `Stream` of retweets may contain multiple retweet cascades.
    fn find_possible_influences(&self, retweets: Stream<G, Tweet>) -> Stream<G, InfluenceEdge<UserID>>;
}

impl<G: Scope> FindPossibleInfluences<G> for Stream<G, (UserID, Vec<UserID>)>
    where G::Timestamp: Hash {
    fn find_possible_influences(&self, retweets: Stream<G, Tweet>) -> Stream<G, InfluenceEdge<UserID>> {
        // For each user, given by their ID, the set of their friends, given by their ID.
        let mut edges = SocialGraph::new();

        self.binary_stream(
            &retweets,
            Exchange::new(|edge: &(UserID, Vec<UserID>)| edge.0 as u64),
            Exchange::new(|retweet: &Tweet| retweet.user.id as u64),
            "FindPossibleInfluences",
            move |friendships, retweets, output| {
                // Input 1: Capture all friends for each user.
                friendships.for_each(|_time, friendship_data| {
                    for friendship in friendship_data.take().iter() {
                        let friendship_set: &mut Vec<UserID> = edges.entry(friendship.0)
                            .or_insert_with(|| Vec::with_capacity(friendship.1.len()));
                        friendship_set.extend(friendship.1.iter());
                        friendship_set.shrink_to_fit();
                        friendship_set.sort()
                    };

                    edges.shrink_to_fit();
                });

                // Input 2: Process the retweets.
                retweets.for_each(|time, retweet_data| {
                    let mut session = output.session(&time);
                    for retweet in retweet_data.take().iter() {
                        // Skip all tweets that are not retweets.
                        let original_tweet: &Tweet = match retweet.retweeted_status {
                            Some(ref t) => t,
                            None => continue
                        };

                        // Get the user's friends.
                        let friends = match edges.get(&retweet.user.id) {
                            Some(friends) => friends,
                            None => continue
                        };

                        // Pass on the possible influence edges.
                        for &friend in friends {
                            let influence = InfluenceEdge::new(friend, retweet.user.id, retweet.created_at, retweet.id,
                                                               original_tweet.id, original_tweet.user.id);
                            session.give(influence);
                        }
                    }
                });
            }
        )
    }
}
