// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Representations of Twitter users.

use abomonation::Abomonation;

/// Users can be anyone or anything.
///
/// Users tweet, follow, create lists, have a home timeline, can be mentioned, and can be looked up in bulk.
///
/// This struct's fields correspond directly to the fields of the same name in the Twitter API.
///
/// # See Also
/// https://dev.twitter.com/overview/api/users
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    /// Integer representation of the unique identifier for this user.
    pub id: u64,

    /// The screen name, handle, or alias that this user identifies themselves with. Screen names are unique, but
    /// subject to change.
    pub screen_name: String
}

unsafe_abomonate!(User : id, screen_name);
