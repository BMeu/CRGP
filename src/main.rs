extern crate ccgp;
extern crate serde_json;
extern crate stopwatch;
extern crate timely;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;

use stopwatch::Stopwatch;
use timely::dataflow::*;
use timely::dataflow::operators::*;

use ccgp::social_graph::edge::*;
use ccgp::timely_operators::*;
use ccgp::twitter::*;

fn main() {
    print_usage(&std::env::args().nth(0).unwrap());

    // Parse the arguments.
    let friendship_dataset = std::env::args().nth(1).unwrap();
    let retweet_dataset = std::env::args().nth(2).unwrap();
    let batch_size: usize = std::env::args().nth(3).unwrap().parse().unwrap();
    let print_result: bool = std::env::args().nth(4).unwrap().parse().unwrap();

    timely::execute_from_args(std::env::args().skip(4), move |computation| {
        let index = computation.index();
        let mut stopwatch = Stopwatch::start_new();

        /******************
         * DATAFLOW GRAPH *
         ******************/

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
                .inspect(move |x| {
                    if print_result {
                        println!("Worker {}: {:?}", index, x);
                    }
                })
                .probe().0;

            (graph_input, retweet_input, probe)
        });
        let time_to_setup: i64 = stopwatch.elapsed_ms();



        /****************
         * SOCIAL GRAPH *
         ****************/

        // Load the social graph from a file into the computation (only on the first worker).
        stopwatch.restart();
        let mut number_of_friendships: u64 = 0;
        if index == 0 {
            let friendship_file = File::open(&friendship_dataset).expect("Could not open friendship dataset.");
            let friendship_file = BufReader::new(friendship_file);

            // Each line contains all friendships of a single user.
            for line in friendship_file.lines().filter_map(|l| l.ok()) {
                let user: Vec<&str> = line.split(':').collect();
                if user.len() == 0 {
                    continue;
                }

                let user_id: u64 = user[0].parse().unwrap();

                let has_friends = user.len() > 1 && !user[1].is_empty();
                if !has_friends {
                    continue;
                }

                for friend_id in user[1].split(',').map(|f| f.parse::<u64>().unwrap()) {
                    number_of_friendships += 1;
                    graph_input.send(DirectedEdge::new(user_id, friend_id));
                }
            }
        }

        // Process the entire social graph before continuing.
        let next_graph = graph_input.epoch() + 1;
        let next_retweets = retweet_input.epoch() + 1;
        graph_input.advance_to(next_graph);
        retweet_input.advance_to(next_retweets);
        while probe.lt(graph_input.time()) {
            computation.step();
        }
        let time_to_process_social_network: i64 = stopwatch.elapsed_ms();



        /************
         * RETWEETS *
         ************/

        // Load and process the retweets.
        stopwatch.restart();
        let mut number_of_retweets: usize = 0;
        if index == 0 {
            let retweet_file = File::open(&retweet_dataset).expect("Could not open retweet file.");
            let retweet_file = BufReader::new(retweet_file);

            for line in retweet_file.lines().map(|l| serde_json::from_str::<Tweet>(&l.expect("{}")).unwrap()) {
                retweet_input.send(line);

                let is_batch_complete: bool = number_of_retweets % batch_size == (batch_size - 1);
                if is_batch_complete {
                    let next_graph = graph_input.epoch() + 1;
                    let next_retweets = retweet_input.epoch() + 1;
                    graph_input.advance_to(next_graph);
                    retweet_input.advance_to(next_retweets);
                    while probe.lt(retweet_input.time()) {
                        computation.step();
                    }
                }

                number_of_retweets += 1;
            }
            let next_graph = graph_input.epoch() + 1;
            let next_retweets = retweet_input.epoch() + 1;
            graph_input.advance_to(next_graph);
            retweet_input.advance_to(next_retweets);
            while probe.lt(retweet_input.time()) {
                computation.step();
            }
        }
        let time_to_process_retweets: i64 = stopwatch.elapsed_ms();



        /***********
         * RESULTS *
         ***********/

        if index == 0 {
            println!();
            println!("Results:");
            println!("  #Friendships: {}", number_of_friendships);
            println!("  #Retweets: {}", number_of_retweets);
            println!("  Batch Size: {}", batch_size);
            println!();
            println!("  Time to set up the computation: {}ms", time_to_setup);
            println!("  Time to load and process the social network: {}ms", time_to_process_social_network);
            println!("  Time to load and process the retweets: {}ms", time_to_process_retweets);
            println!("  Total time: {}ms", time_to_setup + time_to_process_social_network + time_to_process_retweets);
        }
    }).unwrap();
}

fn print_usage(name: &str) {
    println!("Usage: {} <Friend Data Set: Path> <Retweet Data Set: Path> <Batch Size: Int> <Print Result: Bool> [Timely Options]", name);
    println!();
}
