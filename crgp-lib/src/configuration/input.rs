// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Configuration for input sources.

use std::fmt;

use configuration::S3;

/// Configuration of an input source, for either social graph or cascade data sets.
///
/// Supports AWS S3.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InputSource {
    /// Path to the input file.
    pub path: String,

    /// Optionally, configuration to access AWS S3.
    pub s3: Option<S3>,

    /// Private field to prevent initialization without the provided methods.
    ///
    /// All other fields should be public for easy access without getter functions. However, adding more fields later
    /// could break code if the `InputSource` were manually initialized.
    #[serde(skip_serializing)]
    _prevent_outside_initialization: bool,
}

impl InputSource {
    /// Initialize a new input source from a path. The AWS S3 configuration will be set to `None`.
    pub fn new(path: &str) -> InputSource {
        InputSource {
            path: String::from(path),
            s3: None,
            _prevent_outside_initialization: true,
        }
    }

    /// Set the AWS S3 configuration.
    pub fn s3(mut self, s3_configuration: Option<S3>) -> InputSource {
        self.s3 = s3_configuration;
        self
    }
}

impl fmt::Display for InputSource {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.s3 {
            Some(ref s3) => write!(formatter, "{path} on S3 {s3}", path = self.path, s3 = s3),
            None => write!(formatter, "{path}", path = self.path)
        }
    }
}

#[cfg(test)]
mod tests {
    use configuration::S3;
    use super::*;

    #[test]
    fn new() {
        let input = InputSource::new("path/to/source");
        assert_eq!(input.path, String::from("path/to/source"));
        assert_eq!(input.s3, None);
        assert!(input._prevent_outside_initialization);
    }

    #[test]
    fn s3() {
        let s3_config = S3::new("bucket", "region");
        let input = InputSource::new("path/to/source")
            .s3(Some(s3_config.clone()));
        assert_eq!(input.path, String::from("path/to/source"));
        assert_eq!(input.s3, Some(s3_config));
        assert!(input._prevent_outside_initialization);
    }

    #[test]
    fn fmt_display_no_s3() {
        let input = InputSource::new("path/to/source");
        assert_eq!(format!("{}", input), String::from("path/to/source"));
    }

    #[test]
    fn fmt_display_with_s3() {
        let s3_config = S3::new("bucket", "region");
        let input = InputSource::new("path/to/source")
            .s3(Some(s3_config.clone()));
        assert_eq!(format!("{}", input), format!("path/to/source on S3 {}", s3_config));
    }
}
