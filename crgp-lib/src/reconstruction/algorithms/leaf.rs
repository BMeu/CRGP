// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The `LEAF` algorithm.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use timely::dataflow::operators::Filter;
use timely::dataflow::operators::Input;
use timely::dataflow::operators::Probe;
use timely::dataflow::operators::exchange::Exchange;

use OutputTarget;
use UserID;
use reconstruction::algorithms::GraphHandle;
use reconstruction::algorithms::ProbeHandle;
use reconstruction::algorithms::RetweetHandle;
use reconstruction::algorithms::Scope;
use social_graph::InfluenceEdge;
use timely_extensions::operators::FindPossibleInfluences;
use timely_extensions::operators::Write;

/// The `LEAF` algorithm: **L**ocal **E**dges, **A**ctivations, and **F**iltering
///
/// 1. Send all friendship egdes (`(u1, u2)`, `u1` follows `u2`) to the worker destined to store `u1`.
/// 2. Send the current Retweet `r*` made by user `u*` to the worker `w*` storing `u*`'s friendships.
/// 3. On `w*`:
///     1. Mark `u*` as active for this cascade.
///     2. For all friends `u'` of `u*`, create possible influences from `u'` to `u*` for this cascade.
///     3. Send each possible influence to the worker `w'` storing `u'` friendships.
/// 4. On `w'`: produce an actual influence from the possible influence if:
///     1. `u'` has been activated before the Retweet occurred, or
///     2. `u'` is the poster of the original Tweet.
pub fn computation<'a>(scope: &mut Scope<'a>, output: OutputTarget) -> (GraphHandle, RetweetHandle, ProbeHandle) {
    // Create the inputs.
    let (graph_input, graph_stream) = scope.new_input();
    let (retweet_input, retweet_stream) = scope.new_input();

    // For each cascade, given by its ID, a set of activated users, given by their ID, i.e.
    // those users who have retweeted within this cascade before, per worker. Since this map
    // is required within two closures, dynamic borrow checks are required.
    let activations_influences: Rc<RefCell<HashMap<u64, HashMap<UserID, u64>>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let activations_possible_influences = activations_influences.clone();

    // The actual algorithm.
    let probe = graph_stream
        .find_possible_influences(retweet_stream, activations_possible_influences)
        .exchange(|influence: &InfluenceEdge<UserID>| influence.influencer as u64)
        .filter(move |influence: &InfluenceEdge<UserID>| {
            let is_influencer_activated: bool = match activations_influences.borrow()
                .get(&influence.cascade_id)
                {
                    Some(users) => match users.get(&influence.influencer) {
                        Some(activation_timestamp) => &influence.timestamp > activation_timestamp,
                        None => false
                    },
                    None => false
                };
            let is_influencer_original_user: bool = influence.influencer == influence.original_user;

            is_influencer_activated || is_influencer_original_user
        })
        .write(output)
        .probe().0;

    (graph_input, retweet_input, probe)
}
