extern crate ccgp;
extern crate stopwatch;
extern crate timely;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use stopwatch::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::operators::*;

use ccgp::*;
use ccgp::social_graph::edge::*;
use ccgp::timely_operators::*;

fn main() {
    // Determine which data sets to use and the batch size.
    let friendship_dataset = std::env::args().nth(1).unwrap();
    let retweet_dataset = std::env::args().nth(2).unwrap();
    let batch_size: usize = std::env::args().nth(3).unwrap().parse().unwrap();

    timely::execute_from_args(std::env::args().skip(3), move |computation| {
        println!("Batch Size: {}", batch_size);

        let mut stopwatch = Stopwatch::start_new();
        let index = computation.index();

        // Load the social graph, but only on the first worker.
        let friendships: HashSet<DirectedEdge<u64>> = if index == 0 {
            let friendships = social_graph::load::from_file(&friendship_dataset);
            println!("Time to load social network: {}", stopwatch);
            println!("#Friendships: {}", friendships.len());
            friendships
        } else {
            HashSet::new()
        };
        let number_of_friendships = friendships.len();
        stopwatch.restart();

        // Load the retweets, but only on the first worker.
        let retweets: Vec<twitter::Tweet> = if index == 0 {
            let retweets = twitter::load::from_file(&retweet_dataset);
            println!("Time to load retweets: {}", stopwatch);
            println!("#Retweets: {}", retweets.len());
            retweets
        } else {
            vec![]
        };
        let number_of_retweets = retweets.len();
        stopwatch.restart();

        // Reconstruct the cascade.
        // Algorithm:
        // 1. Send all friendship edges (u1 -> u2, u1 follows u2) to respective workers (based on u1).
        // 2. Send a retweet made by u* to the worker where u*'s friendship edges are.
        // 3. On this worker: mark u* and the original user u as active for this cascade.
        // 4. On this worker: for all friends of u*, create (possible) influence edges (PIE) for this
        //    cascade, from the friend u' to u*, with timestamp of u*'s retweet.
        // 5. Send each PIE to the worker which stores u'.
        // 6. On this worker: filter all PIEs, output only those where u' has been activated before.
        let (mut graph_input, mut retweet_input, probe) = computation.scoped::<u64, _, _>(move |scope| {

            // Create the inputs.
            let (graph_input, graph_stream) = scope.new_input();
            let (retweet_input, retweet_stream) = scope.new_input();

            // For each cascade, given by its ID, a set of activated users, given by their ID, i.e.
            // those users who have retweeted within this cascade before, per worker. Since this map
            // is required within two closures, dynamic borrow checks are required.
            let activations_influences: Rc<RefCell<HashMap<u64, HashSet<u64>>>> = Rc::new(RefCell::new(HashMap::new()));
            let activations_possible_influences = activations_influences.clone();

            let probe = graph_stream
                .find_possible_influences(retweet_stream, activations_possible_influences)
                .exchange(|influence: &InfluenceEdge<u64>| influence.influencer)
                .filter(move |influence: &InfluenceEdge<u64>| {
                    match activations_influences.borrow().get(&influence.cascade_id) {
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
        let mut round = 0;
        for retweet in retweets {
            retweet_input.send(retweet);

            // Process the batch of retweets.
            let is_batch_complete: bool = round % batch_size == (batch_size - 1);
            let is_last_retweet: bool = round == (number_of_retweets - 1);
            if is_batch_complete || is_last_retweet {
                let next_graph = graph_input.epoch() + 1;
                let next_retweets = retweet_input.epoch() + 1;
                graph_input.advance_to(next_graph);
                retweet_input.advance_to(next_retweets);
                while probe.lt(retweet_input.time()) {
                    computation.step();
                }
            }

            round += 1;
        }
        if index == 0 {
            println!("Time to process retweets: {}", stopwatch);
            println!("#Friendships: {}", number_of_friendships);
            println!("#Retweets: {}", number_of_retweets);
            println!("Batch Size: {}", batch_size);
        }
    }).unwrap();
}
