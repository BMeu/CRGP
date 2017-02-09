//! Load a social graph from various sources.

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use super::DirectedEdge;

/// Load a social graph from a text file.
///
/// For each user, the file contains a single line. The user's ID comes first and then, separated
/// by a colon, a comma-separated list of the IDs of the user's friends. All IDs must be integers,
/// no whitespace (except line-breaks).
///
/// # Examples
///
/// The file:
///
/// ```text
/// # user_id:friend1_id,friend2_id,...
/// 0:1,2
/// 1:0,2
/// 2:0
/// 3:2
/// ```
///
/// will result in the following directed edges:
///
/// ```text
/// (0, 1)
/// (0, 2)
/// (1, 0)
/// (1, 2)
/// (2, 0)
/// (3, 2)
/// ```
pub fn from_file<P>(filename: P) -> HashSet<DirectedEdge<u64>>
    where P: AsRef<Path> {
    let file = File::open(filename).expect("Could not open file.");
    let file = BufReader::new(file);

    let users: Vec<String> = file.lines()
        .map(|line| line.expect("Error"))
        .collect();

    let mut friendships: HashSet<DirectedEdge<u64>> = HashSet::new();
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
                let friendship = DirectedEdge::new(user_id, friend_id);
                friendships.insert(friendship);
            }
        }
    }

    friendships
}
