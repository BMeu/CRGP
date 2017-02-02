extern crate ccgp;
extern crate stopwatch;
extern crate timely;

use std::collections::HashSet;
use std::mem;

use stopwatch::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::operators::*;
use timely::dataflow::operators::aggregation::Aggregate;

use ccgp::*;
use ccgp::timely_operators::*;

fn main() {
    timely::execute_from_args(std::env::args(), |computation| {
        let mut stopwatch = Stopwatch::start_new();
        let index = computation.index();

        // Load the social graph, but only on the first worker.
        let friendship_dataset = "data/friends_test.txt";
        let friendships: HashSet<Edge<u64>> = if index == 0 {
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

        // Determine the user in the social graph who has the most followers.
        let (mut input, probe) = computation.scoped::<u64, _, _>(move |scope| {

            // Create the inputs.
            let (input, stream) = scope.new_input();

            let probe = stream
                .exchange(|edge: &Edge<u64>| hash(&edge.0))                 // Partition the graph among all workers.
                .map_in_place(|edge| mem::swap(&mut edge.0, &mut edge.1))   // Invert edges.
                .aggregate(                                                 // How many followers does each user have?
                    |_user, _follower, num_followers| { *num_followers += 1; },
                    |user, num_followers: u64| (user, num_followers),
                    |user| *user as u64
                )
                .max()              // Compute the local maximums.
                .exchange(|_| 0)    // Send all local maximums to a single worker to find the global maximum.
                .max()              // Compute the global maximum.
                .inspect(move |x: &Edge<u64>| println!("User {} has the most followers: {}", x.0, x.1))
                .probe().0;

            (input, probe)
        });

        // Introduce the social graph into the computation.
        stopwatch.restart();
        for friendship in friendships {
            input.send(friendship);
        }

        // Wait until the initial graph has been processed.
        let next = input.epoch() + 1;
        input.advance_to(next);
        while probe.lt(input.time()) {
            computation.step();
        }

        if index == 0 {
            println!("Time to process social network: {}", stopwatch);
        }
    }).unwrap();
}
