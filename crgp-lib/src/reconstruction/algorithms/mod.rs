// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The actual algorithms performing the reconstruction.

use timely::dataflow::operators::input::Handle as InputHandle;
use timely::dataflow::operators::probe::Handle as ProgressHandle;
use timely::dataflow::scopes::Child;
use timely::dataflow::scopes::Root;
use timely::progress::nested::product::Product;
use timely::progress::timestamp::RootTimestamp;
use timely_communication::allocator::Generic;

use UserID;
use twitter::Tweet;

pub mod gale;
pub mod leaf;

/// The timely dataflow handle for introducing friendships into the graph.
pub type GraphHandle = InputHandle<u64, (UserID, Vec<UserID>)>;

/// The timely dataflow handle for getting progress information.
pub type ProbeHandle = ProgressHandle<Product<RootTimestamp, u64>>;

/// The timely dataflow handle for introducing Retweets into the graph.
pub type RetweetHandle = InputHandle<u64, Tweet>;

/// The sub-scope of the dataflow graph containing the actual computation.
pub type Scope<'a> = Child<'a, Root<Generic>, u64>;
