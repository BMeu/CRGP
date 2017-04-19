// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Write a stream to a file.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Write as IOWrite;
use std::io::BufWriter;
use std::path::PathBuf;

use timely::dataflow::{Stream, Scope};
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::unary::Unary;

use social_graph::InfluenceEdge;

/// Write a stream to a file, passing on all seen messages.
pub trait Write<G: Scope> {
    /// Write all input messages to a file in the given `output_directory` and pass them on. If `output_directory` is
    /// `None`, the messages will be written to STDOUT.
    ///
    /// On any IO error, an error log message will be generated using the
    /// [`log`](https://doc.rust-lang.org/log/log/index.html) crate.
    fn write(&self, output_directory: Option<PathBuf>) -> Stream<G, InfluenceEdge<u64>>;
}

impl<G: Scope> Write<G> for Stream<G, InfluenceEdge<u64>>
where G::Timestamp: Hash {
    fn write(&self, output_directory: Option<PathBuf>) -> Stream<G, InfluenceEdge<u64>> {
        // For each cascade, a separate file writer.
        let mut cascade_writers: HashMap<u64, BufWriter<File>> = HashMap::new();

        // For each timely time, a list of the influences seen at that time.
        let mut influences_at_time: HashMap<G::Timestamp, HashSet<InfluenceEdge<u64>>> = HashMap::new();

        self.unary_notify(
            Exchange::new(|influence: &InfluenceEdge<u64>| influence.cascade_id),
            "Write",
            Vec::new(),
            move |influences, output, notificator| {
                // Process the influence edges: immediately pass them on and save them for batched writing.
                influences.for_each(|time, influence_data| {
                    notificator.notify_at(time.clone());

                    let mut influences_now = influences_at_time.entry(time.time())
                        .or_insert(HashSet::new());

                    for ref influence in influence_data.iter() {
                        // Tell the compile the influence edge is of type 'InfluenceEdge<u64>'.
                        let influence: &InfluenceEdge<u64> = influence;

                        influences_now.insert(influence.clone());
                        output.session(&time).give(influence.clone());
                    }
                });

                // If a timely time is done, write all associated edges.
                notificator.for_each(|time, _num, _notify| {
                    // TODO: Find a more elegant way to get rid of borrow conflicts.
                    {
                        let influences_now = match influences_at_time.get(&time) {
                            Some(influences_now) => influences_now,
                            None => return
                        };

                        for influence in influences_now {
                            // Tell the compiler the influence edge is of type 'InfluenceEdge<u64>'.
                            let influence: &InfluenceEdge<u64> = influence;

                            match output_directory {
                                Some(ref directory) => {
                                    // Get the writer for this cascade. If there is none, create it.
                                    let has_writer: bool = cascade_writers.contains_key(&influence.cascade_id);
                                    if !has_writer {
                                        let filename: String = format!("cascs-{id}.csv", id = influence.cascade_id);
                                        let path: PathBuf = directory.join(filename);
                                        let path_c: PathBuf = path.clone();
                                        let file = match File::create(path) {
                                            Ok(file) => file,
                                            Err(message) => {
                                                error!("Could not create {file:?}: {error}",
                                                       file = path_c, error = message);
                                                continue;
                                            }
                                        };
                                        trace!("Created result file {file:?}", file = path_c);
                                        let _ = cascade_writers.insert(influence.cascade_id, BufWriter::new(file));
                                    }
                                    let cascade_writer = cascade_writers.get_mut(&influence.cascade_id);
                                    let cascade_writer: &mut BufWriter<File> = match cascade_writer {
                                        Some(writer) => writer,
                                        None => {
                                            // This should not be possible since the above code will insert a new writer
                                            // if there is none yet.
                                            continue;
                                        }
                                    };

                                    // Write the edge into the writer's buffer.
                                    let _ = writeln!(cascade_writer,
                                                     "{cascade};{retweet};{user};{influencer};{time};-1",
                                                     cascade = influence.cascade_id, retweet = influence.retweet_id,
                                                     user = influence.influencee, influencer = influence.influencer,
                                                     time = influence.timestamp);
                                },
                                None => {
                                    println!("{cascade};{retweet};{user};{influencer};{time};-1",
                                             cascade = influence.cascade_id, retweet = influence.retweet_id,
                                             user = influence.influencee, influencer = influence.influencer,
                                             time = influence.timestamp);
                                }
                            }
                        }
                    }

                    // Finally, remove the influence edges for this time.
                    let _ = influences_at_time.remove(&time);
                });
            }
        )
    }
}
