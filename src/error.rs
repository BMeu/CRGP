//! Error handling.

use std;
use std::fmt;
use std::io;
use std::result;

/// A specialized `Result` type for CRGP.
pub type Result<T> = result::Result<T, Error>;

/// A wrapper type for all errors caused by this crate.
#[derive(Debug)]
pub enum Error {
    /// IO errors caused by file handling failures.
    IO(io::Error),

    /// Errors caused by Timely failures.
    Timely(String)
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref error) => error.fmt(formatter),
            Error::Timely(ref error) => error.fmt(formatter)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref error) => error.description(),
            Error::Timely(ref error) => error
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::IO(ref error) => Some(error),
            Error::Timely(_) => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::Timely(error)
    }
}
