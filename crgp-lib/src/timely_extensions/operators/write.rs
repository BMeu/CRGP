// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Write a stream to a file.

use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write as IOWrite;
use std::io::BufWriter;
use std::path::PathBuf;

use timely::dataflow::Stream;
use timely::dataflow::Scope;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::unary::Unary;

use UserID;
use configuration::OutputTarget;
use social_graph::InfluenceEdge;

/// Write a stream to a file, passing on all seen messages.
pub trait Write<G: Scope> {
    /// Write all input messages to the given `output_target` without producing any output. If `output_target` is
    /// `None`, the messages will be passed on without any further operations.
    ///
    /// On any IO error, an error log message will be generated using the
    /// [`log`](https://doc.rust-lang.org/log/log/index.html) crate.
    fn write(&self, output_target: OutputTarget) -> Stream<G, InfluenceEdge<UserID>>;
}

impl<G: Scope> Write<G> for Stream<G, InfluenceEdge<UserID>>
where G::Timestamp: Hash {
    #[cfg_attr(feature = "cargo-clippy", allow(print_stdout))]
    fn write(&self, output_target: OutputTarget) -> Stream<G, InfluenceEdge<UserID>> {
        // If the output target is None, return an operator that does nothing.
        if let OutputTarget::None = output_target {
            return self.unary_stream(Pipeline, "Write", |_influences, _output| {})
        }

        // For each cascade, a separate file writer.
        let mut cascade_writers: HashMap<u64, BufWriter<File>> = HashMap::new();

        // For each timely time, a list of the influences seen at that time.
        let mut influences_at_time: HashMap<G::Timestamp, Vec<InfluenceEdge<UserID>>> = HashMap::new();

        self.unary_notify(
            Exchange::new(|influence: &InfluenceEdge<UserID>| influence.cascade_id),
            "Write",
            Vec::new(),
            move |influences, _output, notificator| {
                // Process the influence edges: immediately pass them on and save them for batched writing.
                influences.for_each(|time, influence_data| {
                    notificator.notify_at(time.clone());

                    let mut influences_now = influences_at_time.entry(time.time().clone())
                        .or_insert_with(Vec::new);
                    influences_now.extend(influence_data.drain(..));
                });

                // If a timely time is done, write all associated edges.
                notificator.for_each(|time, _num, _notify| {
                    // Introduce this sub-scope to unborrow `influences_at_time` so old entries can be removed from it
                    // at the end.
                    {
                        let influences_now: &Vec<InfluenceEdge<UserID>> = match influences_at_time.get(&time) {
                            Some(influences_now) => influences_now,
                            None => return
                        };

                        for influence in influences_now {
                            // Tell the compiler the influence edge is of type 'InfluenceEdge<u64>'.
                            let influence: &InfluenceEdge<UserID> = influence;

                            match output_target {
                                OutputTarget::Directory(ref directory) => {
                                    let cascade: u64 = influence.cascade_id;

                                    // Create a buffered writer for this edge's cascade if there is none yet.
                                    let has_writer: bool = cascade_writers.contains_key(&cascade);
                                    if !has_writer {
                                        let filename: String = format!("cascs-{id}.csv", id = cascade);
                                        let path: PathBuf = directory.join(filename);

                                        // Create the file (overwrite existing files).
                                        let file: File = match File::create(&path) {
                                            Ok(file) => file,
                                            Err(message) => {
                                                error!("Could not create {file}: {error}",
                                                       file = path.display(), error = message);
                                                continue;
                                            }
                                        };
                                        trace!("Created result file {file}", file = path.display());
                                        let _ = cascade_writers.insert(cascade, BufWriter::new(file));
                                    }

                                    // Get the writer. Failing is impossible since the writer has just been created.
                                    let writer: Option<&mut BufWriter<File>> = cascade_writers.get_mut(&cascade);
                                    let writer: &mut BufWriter<File> = match writer {
                                        Some(writer) => writer,
                                        None => continue,
                                    };

                                    // Write the edge.
                                    let _ = writeln!(writer, "{}", influence);
                                },
                                OutputTarget::StdOut => {
                                    println!("{}", influence);
                                },
                                OutputTarget::None => {}
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
