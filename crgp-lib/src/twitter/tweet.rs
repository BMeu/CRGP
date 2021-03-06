// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Representations of tweets.

use abomonation::Abomonation;

use twitter::User;

/// Tweets are the basic atomic building block of all things Twitter.
///
/// Tweets are also known as "status updates." Tweets can be embedded, replied to, liked, unliked and deleted.
///
/// This struct's fields correspond directly to the fields of the same name in the Twitter API.
///
/// # See Also
/// https://dev.twitter.com/overview/api/tweets
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Tweet {
    /// UTC time when this tweet was created.
    pub created_at: u64,

    /// The integer representation of the unique identifier for this tweet.
    pub id: u64,

    /// The user who posted this tweet.
    pub user: User
}

unsafe_abomonate!(Tweet : created_at, id, user);
