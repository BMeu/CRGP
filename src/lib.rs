//! Reconstruct a retweet cascade.

#![warn(missing_docs)]

extern crate fnv;

use std::collections::HashSet;
use std::fs::File;
use std::hash::*;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

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
