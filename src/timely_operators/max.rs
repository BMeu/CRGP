use std::collections::HashMap;
use std::hash::*;

use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::unary::Unary;

pub trait Max<G: Scope> {
    fn max(&self) -> Stream<G, (u64, u64)>;
}

impl<G: Scope> Max<G> for Stream<G, (u64, u64)>
where G::Timestamp: Hash {
    fn max(&self) -> Stream<G, (u64, u64)> {
        let mut max_per_time = HashMap::new();
        self.unary_notify(Pipeline, "Max", vec![], move |input, output, notificator| {
            input.for_each(|time, data| {
                notificator.notify_at(time.clone());

                // Get the current max or insert and use 0 if no max has been set before.
                let mut max = max_per_time.entry(time.time())
                    .or_insert((0, 0));

                // Determine which local user has the most followers.
                for &datum in data.iter() {
                    let (user, num_followers) = datum;

                    if num_followers > max.1 {
                        *max = (user, num_followers);
                    }
                }
            });

            // Send and remove old maximums.
            notificator.for_each(|time, _num, _notify| {
                let mut session = output.session(&time);
                let max = max_per_time.remove(&time);
                match max {
                    Some(m) => session.give(m),
                    None => {}
                }
            })
        })
    }
}
