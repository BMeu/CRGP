// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Functions for getting Tweets.

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;
use std::path::PathBuf;

use serde_json;

use Error;
use Result;
use twitter::Tweet;

/// Load the retweets from the given `path`.
pub fn from_file(path: &PathBuf) -> Result<Vec<Tweet>> {
    info!("Loading Retweets");
    if !path.is_file() {
        error!("Retweet data set is a not a file: {path}", path = path.display());
        return Err(Error::from(IOError::new(IOErrorKind::InvalidInput,
                                            format!("Retweet data set is not a file: {path}", path = path.display()))));
    }

    // Open the file.
    let retweet_file = match File::open(path.clone()) {
        Ok(file) => file,
        Err(error) => {
            error!("Could not open Retweet data set: {error}", error = error);
            return Err(Error::from(error));
        }
    };
    let retweet_file = BufReader::new(retweet_file);

    // Parse the lines while discarding those that are invalid.
    let retweets: Vec<Tweet> = retweet_file.lines()
        .filter_map(|line: IOResult<String>| -> Option<Tweet> {
            match line {
                Ok(line) => {
                    match serde_json::from_str::<Tweet>(&line) {
                        Ok(tweet) => Some(tweet),
                        Err(message) => {
                            warn!("Failed to parse Tweet: {error}", error = message);
                            None
                        }
                    }
                },
                Err(message) => {
                    warn!("Invalid line in file {file}: {error}", file = path.display(), error = message);
                    None
                }
            }
        })
        .collect();
    Ok(retweets)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use Result;
    use twitter::Tweet;

    #[test]
    fn from_file() {
        // Invalid file.
        let path = PathBuf::from(String::from("../data/retweets.invalid.json"));
        let retweets: Result<Vec<Tweet>> = super::from_file(&path);
        assert!(retweets.is_err());

        // Valid file.
        let path = PathBuf::from(String::from("../data/retweets.json"));
        let retweets: Result<Vec<Tweet>> = super::from_file(&path);
        assert!(retweets.is_ok());
        let retweets: Vec<Tweet> = retweets.expect("Retweet parsing failed, but previous assertion told otherwise.");
        assert_eq!(retweets.len(), 6);

        // The Tweets must be sorted on their timestamp.
        let mut previous_timestamp: u64 = 0;
        for retweet in retweets {
            assert!(retweet.created_at >= previous_timestamp);
            previous_timestamp = retweet.created_at;
        }
    }
}
