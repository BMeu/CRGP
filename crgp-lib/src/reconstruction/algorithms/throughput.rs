// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Not a reconstruction algorithm, but a computation to measure the throughput of messages.

use timely::dataflow::operators::Broadcast;
use timely::dataflow::operators::Input;
use timely::dataflow::operators::Probe;
use timely::dataflow::operators::exchange::Exchange;

use configuration::OutputTarget;
use reconstruction::algorithms::GraphHandle;
use reconstruction::algorithms::ProbeHandle;
use reconstruction::algorithms::RetweetHandle;
use reconstruction::algorithms::Scope;

pub fn computation<'a>(scope: &mut Scope<'a>, _output: OutputTarget) -> (GraphHandle, RetweetHandle, ProbeHandle) {
    // Create the inputs.
    let (graph_input, _graph_stream) = scope.new_input();
    let (retweet_input, retweet_stream) = scope.new_input();

    // The actual algorithm;
    let probe = retweet_stream
        .broadcast()
        .exchange(|_| 0)
        .probe();

    (graph_input, retweet_input, probe)
}
