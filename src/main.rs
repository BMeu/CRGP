extern crate ccgp;
extern crate stopwatch;
extern crate timely;

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;

use stopwatch::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::*;

use ccgp::*;
use ccgp::social_graph::edge::*;
use ccgp::twitter::*;

fn main() {
    timely::execute_from_args(std::env::args(), |computation| {
        let mut stopwatch = Stopwatch::start_new();
        let index = computation.index();

        // Load the social graph, but only on the first worker.
        let friendship_dataset = "data/friends_test.txt";
        let friendships: HashSet<DirectedEdge<u64>> = if index == 0 {
            let friendships = social_graph::load::from_file(friendship_dataset);
            println!("Time to load social network: {}", stopwatch);
            println!("#Friendships: {}", friendships.len());
            friendships
        } else {
            HashSet::new()
        };
        stopwatch.restart();

        // Load the retweets, but only on the first worker.
        let retweet_dataset = "data/cascade_test.json";
        let retweets: Vec<twitter::Tweet> = if index == 0 {
            let retweets = twitter::load::from_file(retweet_dataset);
            println!("Time to load retweets: {}", stopwatch);
            println!("#Retweets: {}", retweets.len());
            retweets
        } else {
            vec![]
        };
        stopwatch.restart();

        // Reconstruct the cascade.
        // Algorithm:
        // 1. Send all friendship edges (u1 -> u2, u1 follows u2) to respective workers (based on u1).
        // 2. Send a retweet of u* to the worker where u*'s friendship edges are.
        // 3. On this worker: mark u* and the original user u as active for this cascade.
        // 4. On this worker: for all friends of u*, create (possible) influence edges (PIE) for this
        //    cascade, from the friend u' to u*, with timestamp of u*'s retweet.
        // 5. Send each PIE to the worker which stores u'.
        // 6. On this worker: filter all PIEs, output only those where u' has been activated before.
        let (mut graph_input, mut retweet_input, probe) = computation.scoped::<u64, _, _>(move |scope| {

            // Create the inputs.
            let (graph_input, graph_stream) = scope.new_input();
            let (retweet_input, retweet_stream) = scope.new_input();

            let mut user_friends: HashMap<u64, HashSet<u64>> = HashMap::new();  // [user, {friends}]
            let activated_users: Rc<RefCell<HashMap<u64, HashSet<u64>>>> = Rc::new(RefCell::new(HashMap::new()));  // [cascade_id, {retweeting_users}]
            let captured_activated_users = activated_users.clone();

            let probe = graph_stream
                .binary_stream(
                    &retweet_stream,
                    Exchange::new(|edge: &DirectedEdge<u64>| hash(&edge.source)),
                    Exchange::new(|retweet: &Tweet| hash(&retweet.user.id)),
                    "Reconstruct",
                    move |edges, retweets, output| {
                        // Input 1: Simply capture for each received user his friends.
                        edges.for_each(|_time, friendship_data| {
                            for ref friends in friendship_data.iter() {
                                user_friends.entry(friends.source)
                                    .or_insert(HashSet::new())
                                    .insert(friends.destination);
                            }
                        });

                        // Input 2: Process the retweets.
                        retweets.for_each(|time, retweet_data| {
                            for ref retweet in retweet_data.iter() {
                                // Skip all tweets that are not retweets.
                                let original_tweet = match retweet.retweeted_status {
                                    Some(ref t) => t,
                                    None => continue
                                };

                                // Mark this user and the original user as active for this cascade.
                                activated_users.borrow_mut()
                                    .entry(original_tweet.id)
                                    .or_insert_with(|| {
                                        let mut users = HashSet::new();
                                        users.insert(original_tweet.user.id);
                                        users
                                    })
                                    .insert(retweet.user.id);

                                // Get the user's friends.
                                let friends = match user_friends.get(&retweet.user.id) {
                                    Some(friends) => friends,
                                    None => continue
                                };

                                // Pass on the possible influence edges.
                                for &friend in friends {
                                    let influence = InfluenceEdge::new(friend, retweet.user.id,
                                        retweet.created_at, original_tweet.id);
                                    output.session(&time).give(influence);
                                }
                            }
                        });
                    }
                )
                .exchange(|influence: &InfluenceEdge<u64>| hash(&(influence.influencer)))
                .filter(move |influence: &InfluenceEdge<u64>| {
                    match captured_activated_users.borrow().get(&influence.cascade_id) {
                        Some(users) => users.contains(&influence.influencer),
                        None => false
                    }
                })
                .inspect(move |x| println!("Worker {}: {:?}", index, x))
                .probe().0;

            (graph_input, retweet_input, probe)
        });

        // Introduce the social graph into the computation.
        stopwatch.restart();
        for friendship in friendships {
            graph_input.send(friendship);
        }

        // Process the entire social graph before continuing.
        let next_graph = graph_input.epoch() + 1;
        let next_retweets = retweet_input.epoch() + 1;
        graph_input.advance_to(next_graph);
        retweet_input.advance_to(next_retweets);
        while probe.lt(graph_input.time()) {
            computation.step();
        }

        if index == 0 {
            println!("Time to process social network: {}", stopwatch);
        }

        // Introduce the retweets into the computation.
        stopwatch.restart();
        for retweet in retweets {
            retweet_input.send(retweet);

            // Process the retweet before continuing.
            let next_graph = graph_input.epoch() + 1;
            let next_retweets = retweet_input.epoch() + 1;
            graph_input.advance_to(next_graph);
            retweet_input.advance_to(next_retweets);
            while probe.lt(retweet_input.time()) {
                computation.step();
            }
        }
        if index == 0 {
            println!("Time to process retweets: {}", stopwatch);
        }
    }).unwrap();
}
