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

use s3::bucket::Bucket;
use s3::error::ErrorKind as S3ErrorKind;
use s3::error::S3Error;
use serde_json;

use Error;
use Result;
use configuration::InputSource;
use twitter::Tweet;

/// Load the Retweets from the given input.
pub fn from_source(input: InputSource) -> Result<Vec<Tweet>> {
    info!("Loading Retweets");
    let path: String = input.path.clone();
    match input.s3 {
        Some(s3_config) => from_aws_s3(&path, &s3_config.get_bucket()?),
        None => from_file(&PathBuf::from(path))
    }
}

/// Load the Retweets from the given `path`.
fn from_file(path: &PathBuf) -> Result<Vec<Tweet>> {
    if !path.is_file() {
        #[cfg(not(test))]
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
    let retweet_file: BufReader<File> = BufReader::new(retweet_file);

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

/// Load the Retweets from the given AWS S3 `bucket`.
fn from_aws_s3(path: &str, bucket: &Bucket) -> Result<Vec<Tweet>> {
    // Load the file from S3.
    let (contents, code): (Vec<u8>, u32) = bucket.get(path)?;
    if code != 200 {
        let message: String = format!("Could not get file \"{file}\" from AWS S3 bucket \"{bucket} (region \
                                       {region})\": HTTP error {code}",
                                      file = path, bucket = bucket.name, region = bucket.region, code = code);
        error!("{}", message);
        return Err(Error::from(S3Error::from_kind(S3ErrorKind::Msg(message))));
    }
    let retweet_file: BufReader<&[u8]> = BufReader::new(&contents);

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
                    warn!("Invalid line in file {file}: {error}", file = path, error = message);
                    None
                }
            }
        })
        .collect();
    Ok(retweets)
}


#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::PathBuf;
    use Result;
    use twitter::Tweet;

    #[test]
    fn from_file() {
        // Invalid file.
        let path = PathBuf::from(String::from("../data/retweets.invalid.json"));
        let retweets: Result<Vec<Tweet>> = super::from_file(&path);
        assert!(retweets.is_err());
        if let Err(message) = retweets {
            assert_eq!(message.description(), "Retweet data set is not a file: ../data/retweets.invalid.json");
        }

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
