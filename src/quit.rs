// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Quit the program with standardized exit codes.

use std::error::Error as StdError;
use std::process;

use crgp_lib::Error;

/// The exit codes returned by the program.
#[derive(Clone, Copy, Debug)]
pub enum ExitCode {
    /// Successful (i.e. expected) execution (Code: `0`).
    Success = 0,

    /// Invalid program parameters (Code: `1`).
    IncorrectUsage = 1,

    /// Failure due to I/O operations (Code: `2`).
    IOFailure = 2,

    /// Failure during logger initialization (Code: `3`).
    LoggerFailure = 3,

    /// Execution failure (Code: `4`).
    ExecutionFailure = 4,

    /// Faulty usage of environment variables (Code: `5`).
    EnvVarFailure = 5,

    /// Failure during AWS S3 access (Code: `6`).
    S3Failure = 6,
}

/// Quit the program execution. The exit code and message are chosen based on `error`.
pub fn fail_from_error(error: Error) -> ! {
    match error {
        Error::IO(message) => {
            fail_with_message(ExitCode::IOFailure, message.description());
        },
        Error::Timely(message) => {
            fail_with_message(ExitCode::ExecutionFailure, &message);
        },
        Error::EnvVar(message) => {
            fail_with_message(ExitCode::EnvVarFailure, message.description());
        }
        Error::S3(message) => {
            fail_with_message(ExitCode::S3Failure, message.description());
        }
    }
}

/// Quit the program execution with the given `exit_code` and an error `message` explaining the exit.
pub fn fail_with_message(exit_code: ExitCode, message: &str) -> ! {
    println!("Error: {description}", description = message);
    process::exit(exit_code as i32)
}

/// Quit the program execution with a `Success` exit code.
pub fn succeed() -> ! {
    process::exit(ExitCode::Success as i32)
}
