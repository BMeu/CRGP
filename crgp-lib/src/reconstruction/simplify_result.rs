// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Methods to simplify the results returned by the computation.

use std::result::Result as StdResult;

use timely_communication::initialize::WorkerGuards;

use Error;
use Result;

/// The result returned from the computation is several layers of nested Result types.
pub trait SimplifyResult<R: Send> {
    /// The `result` returned from the computation is several layers of nested `Result` types. Flatten them to the
    /// expected return type. Return the actual result from the first worker, but only if no worker returned an error.
    fn simplify(self) -> Result<R>;
}

impl<R: Send> SimplifyResult<R> for WorkerGuards<Result<R>> {
    fn simplify(self) -> Result<R> {
        let worker_results: Vec<(usize, Result<R>)> = self.join()
            .into_iter()
            .map(|worker_result: StdResult<Result<R>, String>| {
                // Flatten the nested result types.
                match worker_result {
                    Ok(result) => {
                        match result {
                            Ok(inner) => Ok(inner),
                            Err(error) => Err(error)
                        }
                    },
                    Err(message) => Err(Error::from(message))
                }
            })
            .enumerate()
            .rev()
            .collect();

        // The worker results have been enumerated with their worker's index, and this list has been reversed, i.e. the
        // first worker is now at the end. For all workers but the first one, immediately return their failure as this
        // function's return value if they failed. If none of these workers failed return the result of the first
        // worker.
        for (index, worker_result) in worker_results {
            if index == 0 {
                return worker_result
            } else {
                match worker_result {
                    Ok(_) => continue,
                    Err(_) => return worker_result
                }
            }
        }

        // This could only happen if there were no workers at all.
        Err(Error::from("No workers".to_string()))
    }
}
