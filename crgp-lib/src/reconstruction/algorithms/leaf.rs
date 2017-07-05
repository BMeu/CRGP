// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The `LEAF` algorithm.

use timely::dataflow::operators::Input;
use timely::dataflow::operators::Probe;

use configuration::OutputTarget;
use reconstruction::algorithms::GraphHandle;
use reconstruction::algorithms::ProbeHandle;
use reconstruction::algorithms::RetweetHandle;
use reconstruction::algorithms::Scope;
use timely_extensions::operators::FindPossibleInfluences;
use timely_extensions::operators::PrefixFilter;
use timely_extensions::operators::Write;

/// The `LEAF` algorithm: **L**ocal **E**dges, **A**ctivations, and **F**iltering
///
/// 1. Send all friendship egdes (`(u1, u2)`, `u1` follows `u2`) to the worker destined to store `u1`.
/// 2. Send the current Retweet `r*` made by user `u*` to the worker `w*` storing `u*`'s friendships.
/// 3. On `w*`:
///     2. For all friends `u'` of `u*`, create possible influences from `u'` to `u*` for this cascade.
///     3. Send each possible influence to the worker `w'` storing `u'` friendships.
/// 4. On `w'`:
///     1. Mark `u*` as active for this cascade.
///     2. Produce an actual influence from the possible influence if:
///         1. `u'` has been activated before the Retweet occurred, or
///         2. `u'` is the poster of the original Tweet.
pub fn computation<'a>(scope: &mut Scope<'a>, output: OutputTarget) -> (GraphHandle, RetweetHandle, ProbeHandle) {
    // Create the inputs.
    let (graph_input, graph_stream) = scope.new_input();
    let (retweet_input, retweet_stream) = scope.new_input();

    // The actual algorithm.
    let probe = graph_stream
        .find_possible_influences(retweet_stream)
        .filter()
        .write(output)
        .probe();

    (graph_input, retweet_input, probe)
}
