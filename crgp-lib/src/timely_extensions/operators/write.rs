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
use timely::dataflow::operators::unary::Unary;

use configuration::OutputTarget;
use social_graph::InfluenceEdge;
use twitter::User;

/// Write a stream to a file, passing on all seen messages.
pub trait Write<G: Scope> {
    /// Write all input messages to the given `output_target` without producing any output. If `output_target` is
    /// `None`, the messages will be passed on without any further operations.
    ///
    /// On any IO error, an error log message will be generated using the
    /// [`log`](https://doc.rust-lang.org/log/log/index.html) crate.
    fn write(&self, output_target: OutputTarget) -> Stream<G, InfluenceEdge<User>>;
}

impl<G: Scope> Write<G> for Stream<G, InfluenceEdge<User>>
where G::Timestamp: Hash {
    #[cfg_attr(feature = "cargo-clippy", allow(print_stdout))]
    fn write(&self, output_target: OutputTarget) -> Stream<G, InfluenceEdge<User>> {
        let mut file_writer: Option<BufWriter<File>> = None;

        // For each timely time, a list of the influences seen at that time.
        let mut influences_at_time: HashMap<G::Timestamp, Vec<InfluenceEdge<User>>> = HashMap::new();

        self.unary_notify(
            Exchange::new(|_: &InfluenceEdge<User>| 0),
            "Write",
            Vec::new(),
            move |influences, _output, notificator| {
                // Process the influence edges: immediately pass them on and save them for batched writing.
                influences.for_each(|time, influence_data| {
                    notificator.notify_at(time.clone());

                    let mut influences_now = influences_at_time.entry(time.time().clone())
                        .or_insert_with(Vec::new);
                    for influence in influence_data.iter() {
                        influences_now.push(influence.clone());
                    }
                });

                // If a timely time is done, write all associated edges.
                notificator.for_each(|time, _num, _notify| {
                    // Introduce this sub-scope to unborrow `influences_at_time` so old entries can be removed from it
                    // at the end.
                    {
                        let influences_now: &Vec<InfluenceEdge<User>> = match influences_at_time.get(&time) {
                            Some(influences_now) => influences_now,
                            None => return
                        };

                        for influence in influences_now {
                            // Tell the compiler the influence edge is of type 'InfluenceEdge<u64>'.
                            let influence: &InfluenceEdge<User> = influence;

                            match output_target {
                                OutputTarget::Directory(ref directory) => {
                                    if file_writer.is_none() {
                                        let filename: String = String::from("cascs.csv");
                                        let path: PathBuf = directory.join(filename);
                                        let file: File = match File::create(&path) {
                                            Ok(file) => file,
                                            Err(message) => {
                                                error!("Could not create {file}: {error}",
                                                       file = path.display(), error = message);
                                                continue;
                                            }
                                        };

                                        trace!("Created result file {file}", file = path.display());
                                        file_writer = Some(BufWriter::new(file));
                                    }

                                    // Get the writer. Failing is impossible since the writer has just been created.
                                    let writer: &mut BufWriter<File> = match file_writer {
                                        Some(ref mut writer) => writer,
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
