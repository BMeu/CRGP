// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Representations of Twitter users.

use std::fmt;

use abomonation::Abomonation;

use UserID;

/// Users can be anyone or anything.
///
/// Users tweet, follow, create lists, have a home timeline, can be mentioned, and can be looked up in bulk.
///
/// This struct's fields correspond directly to the fields of the same name in the Twitter API.
///
/// # See Also
/// https://dev.twitter.com/overview/api/users
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct User {
    /// Integer representation of the unique identifier for this user.
    pub id: UserID,
}

impl User {
    /// Initialize a new user with the given ID.
    pub fn new(id: UserID) -> User {
        User {
            id: id,
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{id}", id = self.id)
    }
}

unsafe_abomonate!(User : id);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let user = User::new(42);
        assert_eq!(user.id, 42);
    }

    #[test]
    fn fmt_display() {
        let user = User::new(42);
        let fmt = String::from("42");
        assert_eq!(format!("{}", user), fmt);
    }

    #[test]
    fn sort() {
        let mut unsorted: Vec<User> = vec![
            User::new(5),
            User::new(3),
            User::new(4),
            User::new(2),
            User::new(1),
        ];
        let sorted: Vec<User> = vec![
            User::new(1),
            User::new(2),
            User::new(3),
            User::new(4),
            User::new(5),
        ];

        unsorted.sort();
        assert_eq!(unsorted, sorted);
    }
}
