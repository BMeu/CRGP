// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! A social graph structure with methods similar to Rust's container methods.

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use twitter::User;

/// A social graph structure with methods similar to Rust's container methods.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub struct SocialGraph {
    /// The actual container storing the social graph.
    ///
    /// For each user, a list of their friends.
    graph: HashMap<User, Vec<User>>,
}

impl SocialGraph {
    /// Create an empty `SocialGraph`.
    pub fn new() -> SocialGraph {
        SocialGraph {
            graph: HashMap::new()
        }
    }

    /// Shrink the capacity of the social graph as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.graph.shrink_to_fit();
    }

    /// Get the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: User) -> Entry<User, Vec<User>> {
        self.graph.entry(key)
    }

    /// Return a reference to the value corresponding to the key.
    pub fn get(&self, key: &User) -> Option<&Vec<User>> {
        self.graph.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sg = SocialGraph::new();
        assert_eq!(sg.graph, HashMap::new());
    }

    #[test]
    fn shrink_to_fit() {
        let mut sg = SocialGraph::new();
        sg.graph = HashMap::with_capacity(100);
        let _ = sg.graph.insert(User::new(1), vec![User::new(2)]);
        assert!(sg.graph.capacity() >= 100);

        sg.shrink_to_fit();
        assert!(sg.graph.capacity() >= 1);

        // This assertion could fail in the future, depending on the resize policy.
        assert!(sg.graph.capacity() < 100);
    }

    #[test]
    fn entry() {
        let user = User::new(1);
        let friends: Vec<User> = vec![
            User::new(2),
            User::new(3),
            User::new(4),
        ];

        let mut sg = SocialGraph::new();
        assert_eq!(sg.graph.len(), 0);

        {
            let found_friends: &Vec<User> = sg.entry(user)
                .or_insert(friends.clone());
            assert_eq!(found_friends, &friends);
        }

        assert_eq!(sg.graph.len(), 1);
        assert!(sg.graph.contains_key(&user));
        assert_eq!(sg.graph.get(&user), Some(&friends));
    }

    #[test]
    fn get() {
        let user = User::new(1);
        let friends: Vec<User> = vec![
            User::new(2),
            User::new(3),
            User::new(4),
        ];

        let mut sg = SocialGraph::new();
        assert_eq!(sg.get(&user), None);

        let _ = sg.graph.insert(user.clone(), friends.clone());
        assert_eq!(sg.get(&user), Some(&friends));
    }
}
