// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The `GALE` algorithm.

use timely::dataflow::operators::Broadcast;
use timely::dataflow::operators::Input;
use timely::dataflow::operators::Probe;

use OutputTarget;
use reconstruction::algorithms::GraphHandle;
use reconstruction::algorithms::ProbeHandle;
use reconstruction::algorithms::RetweetHandle;
use reconstruction::algorithms::Scope;
use timely_extensions::operators::Reconstruct;
use timely_extensions::operators::Write;

/// The `GALE` algorithm: **G**lobal **A**ctivations, **L**ocal **E**dges
///
/// 1. Send all friendship egdes (`(u1, u2)`, `u1` follows `u2`) to the worker destined to store `u1`.
/// 2. Broadcast the current Retweet `r*` to all workers.
/// 3. Each worker marks the retweeting user `u*` as active for the Retweet's cascade, and, if this is the first Retweet
///    in the cascade, the original user.
/// 4. The worker storing `u*`'s friends produces the influence edges:
///     1. If `u*` has more friends than there are activated users for this cascade, iterate over the cascade's
///        activations. Otherwise, iterate over `u*`'s friends.
///     2. For the current user `u` in the iteration, produce an influence edge if:
///         1. Only for activation iteration: `u` is a friend of `u*`; and
///         2. (The Retweet occurred after the activation of `u`, or
///         3. `u` is the poster of the original Tweet).
pub fn computation<'a>(scope: &mut Scope<'a>, output: OutputTarget) -> (GraphHandle, RetweetHandle, ProbeHandle) {
    // Create the inputs.
    let (graph_input, graph_stream) = scope.new_input();
    let (retweet_input, retweet_stream) = scope.new_input();

    // The actual algorithm;
    let probe = retweet_stream
        .broadcast()
        .reconstruct(graph_stream)
        .write(output)
        .probe();

    (graph_input, retweet_input, probe)
}
