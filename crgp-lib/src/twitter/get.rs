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

/// Load the retweets from the given path.
pub fn from_file(retweet_dataset: &PathBuf) -> Result<Vec<Tweet>> {
    info!("Loading Retweets");

    let retweet_dataset_c: PathBuf = retweet_dataset.clone();

    if !retweet_dataset.is_file() {
        error!("Retweet data set is a not a file");
        return Err(Error::from(IOError::new(IOErrorKind::InvalidInput, "Retweet data set is not a file")));
    }
    let retweet_file = match File::open(retweet_dataset) {
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
                    warn!("Invalid line in file {file:?}: {error}",
                    file = retweet_dataset_c, error = message);
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
        assert_eq!(retweets.unwrap().len(), 6);
    }
}
