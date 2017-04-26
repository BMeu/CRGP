//! Exit the program execution with standardized exit codes.

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
}

/// Quit the program execution. The exit code and message are chosen based on `error`.
pub fn fail_from_error(error: Error) -> ! {
    match error {
        Error::IO(message) => {
            fail_with_message(ExitCode::IOFailure, message.description());
        },
        Error::Timely(message) => {
            fail_with_message(ExitCode::ExecutionFailure, &message);
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
