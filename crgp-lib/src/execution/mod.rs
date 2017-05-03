// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The actual algorithm for reconstructing retweet cascades.

pub use execution::run::run;

pub mod algorithms;
mod run;
mod simplify_result;

/// An alias for user IDs to improve code legibility.
///
/// If the stored value is negative, the ID belongs to a dummy user who was created to pad the social graph.
pub type UserID = i64;
