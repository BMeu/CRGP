// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! A social graph structure with methods similar to Rust's container methods.

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use UserID;

/// A social graph structure with methods similar to Rust's container methods.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub struct SocialGraph {
    /// The actual container storing the social graph.
    ///
    /// For each user, a list of their friends.
    graph: HashMap<UserID, Vec<UserID>>,
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
    pub fn entry(&mut self, key: UserID) -> Entry<UserID, Vec<UserID>> {
        self.graph.entry(key)
    }

    /// Return a reference to the value corresponding to the key.
    pub fn get(&self, key: &UserID) -> Option<&Vec<UserID>> {
        self.graph.get(key)
    }
}
