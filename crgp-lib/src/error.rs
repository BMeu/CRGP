// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Error handling.

use std;
use std::env::VarError;
use std::fmt;
use std::io;
use std::result;

use s3::error::S3Error;

/// A specialized `Result` type for CRGP.
pub type Result<T> = result::Result<T, Error>;

/// A wrapper type for all errors caused by this crate.
#[derive(Debug)]
pub enum Error {
    /// IO errors caused by file handling failures.
    IO(io::Error),

    /// Errors when working with AWS S3.
    S3(S3Error),

    /// Errors caused by Timely failures.
    Timely(String),

    /// Errors caused when handling environment variables.
    EnvVar(VarError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref error) => error.fmt(formatter),
            Error::S3(ref error) => error.fmt(formatter),
            Error::Timely(ref error) => error.fmt(formatter),
            Error::EnvVar(ref error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref error) => error.description(),
            Error::S3(ref error) => error.description(),
            Error::Timely(ref error) => error,
            Error::EnvVar(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::IO(ref error) => Some(error),
            Error::S3(ref error) => Some(error),
            Error::Timely(_) => None,
            Error::EnvVar(ref error) => Some(error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<S3Error> for Error {
    fn from(error: S3Error) -> Error {
        Error::S3(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::Timely(error)
    }
}

impl From<VarError> for Error {
    fn from(error: VarError) -> Error {
        Error::EnvVar(error)
    }
}

#[cfg(test)]
mod tests {
    use std::env::VarError;
    use std::error::Error as StdError;
    use std::fmt;
    use std::io;
    use s3::error::ErrorKind;
    use s3::error::S3Error;
    use super::*;

    #[test]
    fn fmt() {
        let io_error: io::Error = io::Error::from_raw_os_error(42);
        let fmt: String = String::from(format!("{}", io_error));
        let error: Error = Error::IO(io_error);
        assert_eq!(format!("{}", error), fmt);

        let s3_error: S3Error = S3Error::from_kind(ErrorKind::Msg(String::from("AWS S3")));
        let fmt: String = String::from(format!("{}", s3_error));
        let error: Error = Error::S3(s3_error);
        assert_eq!(format!("{}", error), fmt);

        let error: Error = Error::Timely(String::from("42"));
        assert_eq!(format!("{}", error), "42");

        let var_error: VarError = VarError::NotPresent;
        let fmt: String = String::from(format!("{}", var_error));
        let error: Error = Error::EnvVar(var_error);
        assert_eq!(format!("{}", error), fmt);
    }

    #[test]
    fn description() {
        let io_error: io::Error = io::Error::from_raw_os_error(42);
        let description: String = String::from(io_error.description());
        let error: Error = Error::IO(io_error);
        assert_eq!(error.description(), description);

        let s3_error: S3Error = S3Error::from_kind(ErrorKind::Msg(String::from("AWS S3")));
        let description: String = String::from(s3_error.description());
        let error: Error = Error::S3(s3_error);
        assert_eq!(error.description(), description);

        let error: Error = Error::Timely(String::from("42"));
        assert_eq!(error.description(), String::from("42"));

        let var_error: VarError = VarError::NotPresent;
        let description: String = String::from(var_error.description());
        let error: Error = Error::EnvVar(var_error);
        assert_eq!(error.description(), description);
    }

    #[test]
    fn cause() {
        let error: Error = Error::IO(io::Error::from_raw_os_error(42));
        assert!(error.cause().is_some());

        let error: Error = Error::S3(S3Error::from_kind(ErrorKind::Msg(String::from("AWS S3"))));
        assert!(error.cause().is_some());

        let error: Error = Error::Timely(String::from("42"));
        assert!(error.cause().is_none());

        let error: Error = Error::EnvVar(VarError::NotPresent);
        assert!(error.cause().is_some());
    }

    #[test]
    fn from_io() {
        let io_error = io::Error::from_raw_os_error(42);
        assert!(match Error::from(io_error) {
            Error::IO(_) => true,
            _ => false
        });
    }

    #[test]
    fn from_s3() {
        let s3_error = S3Error::from_kind(ErrorKind::Msg(String::from("AWS S3")));
        assert!(match Error::from(s3_error) {
            Error::S3(_) => true,
            _ => false
        });
    }

    #[test]
    fn from_string() {
        let string_error = String::from("42");
        assert!(match Error::from(string_error) {
            Error::Timely(_) => true,
            _ => false
        });
    }

    #[test]
    fn from_var() {
        let var_error = VarError::NotPresent;
        assert!(match Error::from(var_error) {
            Error::EnvVar(_) => true,
            _ => false
        });
    }
}
