extern crate cascadereconstruction_graphparallel;
extern crate stopwatch;

use stopwatch::Stopwatch;

use cascadereconstruction_graphparallel::*;

fn main() {
    let stopwatch = Stopwatch::start_new();

    let friendships = load_social_network_from_file("data/friends.txt");

    println!("Time to load social network: {}", stopwatch);
    println!("#Friendships: {}", friendships.len());
}
