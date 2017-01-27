extern crate cascadereconstruction_graphparallel;
extern crate stopwatch;
extern crate timely;

use std::collections::{HashMap, HashSet};
use std::mem;

use stopwatch::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::operators::*;
use timely::dataflow::operators::aggregation::Aggregate;
use timely::dataflow::channels::pact::Pipeline;

use cascadereconstruction_graphparallel::*;

fn main() {
    timely::execute_from_args(std::env::args(), |computation| {
        // let mut counts_by_time = HashMap::new();
        let mut stopwatch = Stopwatch::start_new();
        let index = computation.index();

        // Load the social graph, but only on the first worker.
        let dataset = "data/friends.txt";
        let friendships: HashSet<Edge<u64>> = if index == 0 {
            let friendships = load_social_network_from_file(dataset);
            println!("Time to load social network: {}", stopwatch);
            println!("#Friendships: {}", friendships.len());
            friendships
        } else {
            HashSet::new()
        };


        let (mut input, probe) = computation.scoped::<u64, _, _>(move |scope| {
            let mut max_per_time = HashMap::new();

            let (input, stream) = scope.new_input();
            let probe = stream.exchange(|edge: &Edge<u64>| hash(&edge.0))
                .map_in_place(|edge| mem::swap(&mut edge.0, &mut edge.1)) // Invert edges.
                .aggregate(  // How many followers does a user have?
                    |_user, _follower, num_followers| { *num_followers += 1; },
                    |user, num_followers: u64| (user, num_followers),
                    |user| *user as u64
                )
                .unary_notify(Pipeline, "Max", vec![], move |input, output, notificator| { // Find the maximum number of followers.
                    input.for_each(|time, data| {
                        notificator.notify_at(time.clone());

                        // Get the current max or insert and use 0 if no max has been set before.
                        let mut max = max_per_time.entry(time.time())
                            .or_insert((0, 0));

                        for &datum in data.iter() {
                            let (user, num_followers) = datum;

                            if num_followers > max.1 {
                                *max = (user, num_followers);
                            }
                        }
                    });

                    // Remove old max's.
                    notificator.for_each(|time, _num, _notify| {
                        let mut session = output.session(&time);
                        let max = max_per_time.remove(&time);
                        match max {
                            Some(m) => session.give(m),
                            None => {}
                        }
                    })
                })
                .inspect(move |x| println!("Worker {}: {:?}", index, x))
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
