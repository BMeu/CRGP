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
use ccgp::social_graph::DirectedEdge;

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
                    Exchange::new(|retweet: &(u64, (u64, u64))| hash(&retweet.0)),
                    "Reconstruct",
                    move |edges, retweets, output| {
                        // Input 1.
                        edges.for_each(|_time, friendship_data| {
                            for ref friends in friendship_data.iter() {
                                user_friends.entry(friends.source)
                                    .or_insert(HashSet::new())
                                    .insert(friends.destination);
                            }
                        });

                        // Input 2.
                        retweets.for_each(|time, retweet_data| {
                            for &retweet in retweet_data.iter() {
                                let (user, original_tweet) = retweet;

                                // Mark this user as active for this cascade.
                                activated_users.borrow_mut()
                                    .entry(original_tweet.0)
                                    .or_insert_with(|| {
                                        let mut users = HashSet::new();
                                        users.insert(original_tweet.1);
                                        users
                                    })
                                    .insert(user);

                                // Get the user's friends.
                                let friends = match user_friends.get(&user) {
                                    Some(friends) => friends,
                                    None => continue
                                };

                                // Pass on the possible edges.
                                for &friend in friends {
                                    let edge = DirectedEdge::new(user, friend);
                                    output.session(&time).give((edge, original_tweet.0));
                                }
                            }
                        });
                    }
                )
                .exchange(|edge: &(DirectedEdge<u64>, u64)| hash(&(edge.0).destination))
                .filter(move |data: &(DirectedEdge<u64>, u64)| {
                    let (ref edge, original_id) = *data;

                    match captured_activated_users.borrow_mut().get(&original_id) {
                        Some(users) => users.contains(&edge.destination),
                        None => false
                    }
                })
                .map(|data: (DirectedEdge<u64>, u64)| ((data.0).destination, (data.0).source))
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
            let original = match retweet.retweeted_status {
                Some(original_tweet) => (original_tweet.id, original_tweet.user.id),
                None => continue
            };
            let retweet = (retweet.user.id, original);
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
        println!("Time to process retweets: {}", stopwatch);
    }).unwrap();
}
