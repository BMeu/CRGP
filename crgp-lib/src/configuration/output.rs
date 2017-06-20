// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Configuration for where to write results.

use std::fmt;
use std::path::PathBuf;

/// Specify where the result will be written to.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OutputTarget {
    /// Write the result to a file in the specified directory.
    Directory(PathBuf),

    /// Write the result to `STDOUT`.
    StdOut,

    /// Do not write the result at all.
    None,
}

impl fmt::Display for OutputTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let target: &str = match *self {
            OutputTarget::Directory(ref path) => return write!(formatter, "\"{path}\"", path = path.display()),
            OutputTarget::StdOut => "STDOUT",
            OutputTarget::None => "[disabled]",
        };
        write!(formatter, "{output}", output = target)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn fmt_display_directory() {
        let output = OutputTarget::Directory(PathBuf::from(String::from("path/to/dir")));
        assert_eq!(format!("{}", output), String::from("\"path/to/dir\""));
    }

    #[test]
    fn fmt_display_stdout() {
        let output = OutputTarget::StdOut;
        assert_eq!(format!("{}", output), String::from("STDOUT"));
    }

    #[test]
    fn fmt_display_disabled() {
        let output = OutputTarget::None;
        assert_eq!(format!("{}", output), String::from("[disabled]"));
    }
}
