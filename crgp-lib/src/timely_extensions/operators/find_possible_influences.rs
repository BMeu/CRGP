// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Find possible influence edges.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::*;
use std::rc::Rc;

use timely::dataflow::Scope;
use timely::dataflow::Stream;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::binary::Binary;

use social_graph::InfluenceEdge;
use social_graph::SocialGraph;
use twitter::Retweet;
use twitter::Tweet;
use twitter::User;

/// Find possible influence edges within social graphs.
pub trait FindPossibleInfluences<G: Scope> {
    /// Find all possible influence edges within a social graph, distinguishing between cascades.
    ///
    /// For a social graph, determine all possible influences for a retweet within that specific
    /// retweet cascade. The `Stream` of retweets may contain multiple retweet cascades.
    fn find_possible_influences(&self, retweets: Stream<G, Retweet>,
                                activated_users: Rc<RefCell<HashMap<u64, HashMap<User, u64>>>>)
                                -> Stream<G, InfluenceEdge<User>>;
}

impl<G: Scope> FindPossibleInfluences<G> for Stream<G, (User, Vec<User>)>
    where G::Timestamp: Hash {
    fn find_possible_influences(&self, retweets: Stream<G, Retweet>,
                                activated_users: Rc<RefCell<HashMap<u64, HashMap<User, u64>>>>)
                                -> Stream<G, InfluenceEdge<User>> {
        // For each user, given by their ID, the set of their friends, given by their ID.
        let mut edges = SocialGraph::new();

        self.binary_stream(
            &retweets,
            Exchange::new(|edge: &(User, Vec<User>)| edge.0.id as u64),
            Exchange::new(|retweet: &Retweet| retweet.user.id as u64),
            "FindPossibleInfluences",
            move |friendships, retweets, output| {
                // Input 1: Capture all friends for each user.
                friendships.for_each(|_time, friendship_data| {
                    for friendship in friendship_data.drain(..) {
                        let user: User = friendship.0;
                        let friends: Vec<User> = friendship.1;

                        let friendship_set: &mut Vec<User> = edges.entry(user)
                            .or_insert_with(|| Vec::with_capacity(friends.len()));
                        friendship_set.extend(friends);
                        friendship_set.shrink_to_fit();
                        friendship_set.sort()
                    };

                    edges.shrink_to_fit();
                });

                // Input 2: Process the retweets.
                retweets.for_each(|time, retweet_data| {
                    let mut session = output.session(&time);
                    for retweet in retweet_data.take().iter() {
                        let original_tweet: &Tweet = &retweet.retweeted_status;

                        // Mark this user and the original user as active for this cascade.
                        let _ = activated_users.borrow_mut()
                            .entry(original_tweet.id)
                            .or_insert_with(HashMap::new)
                            .entry(retweet.user)
                            .or_insert(retweet.created_at);

                        // Get the user's friends.
                        let friends = match edges.get(&retweet.user) {
                            Some(friends) => friends,
                            None => continue
                        };

                        // Pass on the possible influence edges.
                        for &friend in friends {
                            let influence = InfluenceEdge::new(friend, retweet.user, retweet.created_at, retweet.id,
                                                               original_tweet.id, original_tweet.user);
                            session.give(influence);
                        }
                    }
                });
            }
        )
    }
}
