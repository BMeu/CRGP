//! Reconstruct a retweet cascade.

#![warn(missing_docs)]

extern crate fnv;
extern crate timely;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::*;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use timely::Data;
use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::message::Content;
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::unary::Unary;

/// An edge in a graph.
pub type Edge<T> = (T, T);

/// Hash an item.
pub fn hash<T: Hash>(item: &T) -> u64 {
    let mut h: fnv::FnvHasher = Default::default();
    item.hash(&mut h);
    h.finish()
}

/// Expected lines: user_id:friend1_id,friend2_id,... (IDs are integers).
pub fn load_social_network_from_file<P>(filename: P) -> HashSet<Edge<u64>>
    where P: AsRef<Path> {
    let file = File::open(filename).expect("Could not open file.");
    let file = BufReader::new(file);

    let users: Vec<String> = file.lines()
        .map(|line| line.expect("Error"))
        .collect();

    let mut friendships: HashSet<Edge<u64>> = HashSet::new();
    for user in users {
        let user: Vec<&str> = user.split(':').collect();
        if user.len() == 0 {
            continue;
        }

        let user_id: u64 = user[0].parse().unwrap();

        let has_friends = user.len() > 1 && !user[1].is_empty();
        if has_friends {
            let friends: Vec<u64> = user[1].split(',')
                .map(|friend| friend.parse().unwrap())
                .collect();

            for friend_id in friends {
                let friendship: Edge<u64> = (user_id, friend_id);
                friendships.insert(friendship);
            }
        }
    }

    friendships
}

pub trait Max<G: Scope> {
    fn max(&self) -> Stream<G, (u64, u64)>;
}

impl<G: Scope> Max<G> for Stream<G, (u64, u64)>
where G::Timestamp: Hash {
    fn max(&self) -> Stream<G, (u64, u64)> {
        let mut max_per_time = HashMap::new();
        self.unary_notify(Pipeline, "Max", vec![], move |input, output, notificator| {
            input.for_each(|time, data| {
                notificator.notify_at(time.clone());

                // Get the current max or insert and use 0 if no max has been set before.
                let mut max = max_per_time.entry(time.time())
                    .or_insert((0, 0));

                // Determine which local user has the most followers.
                for &datum in data.iter() {
                    let (user, num_followers) = datum;

                    if num_followers > max.1 {
                        *max = (user, num_followers);
                    }
                }
            });

            // Send and remove old maximums.
            notificator.for_each(|time, _num, _notify| {
                let mut session = output.session(&time);
                let max = max_per_time.remove(&time);
                match max {
                    Some(m) => session.give(m),
                    None => {}
                }
            })
        })
    }
}
