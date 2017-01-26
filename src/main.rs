extern crate cascadereconstruction_graphparallel;
extern crate fnv;
extern crate stopwatch;
extern crate timely;

use std::collections::HashSet;
use std::hash::*;

use stopwatch::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::operators::*;
use timely::dataflow::operators::aggregation::Aggregate;

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
            let (input, stream) = scope.new_input();
            let probe = stream.exchange(|edge: &Edge<u64>| hash(&edge.0))
                .map(|edge| (edge.1, edge.0))
                .aggregate(
                    |_key, _val, agg| { *agg += 1; },
                    |key, agg: u64| (key, agg),
                    |key| *key as u64
                )
                .inspect(move |x| println!("User {} has {} followers (Worker {})", x.0, x.1, index))
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

fn hash<T: Hash>(item: &T) -> u64 {
    let mut h: fnv::FnvHasher = Default::default();
    item.hash(&mut h);
    h.finish()
}
