//! Find the maximum in a stream.

use std::collections::HashMap;
use std::hash::*;

use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::unary::Unary;

/// Find the maximum element within a timestamp.
pub trait Max<G: Scope> {
    /// Find the maximum element within a timestamp in a stream of tuples.
    ///
    /// For each tuple in the stream, the tuple's second element will be considered for the maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate ccgp;
    /// extern crate timely;
    ///
    /// use ccgp::timely_operators::Max;
    /// use timely::dataflow::operators::{Capture, ToStream};
    /// use timely::dataflow::operators::capture::Extract;
    /// use timely::progress::timestamp::RootTimestamp;
    ///
    /// # fn main() {
    /// let captured = timely::example(|scope| {
    ///     vec![(2, 1), (1, 2)].to_stream(scope)
    ///         .max()
    ///         .capture()
    /// });
    ///
    /// let extracted = captured.extract();
    /// assert_eq!(extracted, vec![(RootTimestamp::new(0), vec![(1, 2)])]);
    /// # }
    /// ```
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
