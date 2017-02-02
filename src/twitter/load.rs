//! Load a data set of retweets from various sources.

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use serde_json;

use super::Tweet;

/// Load a set of retweets from a JSON file.
///
/// Each line consists of a single retweet in JSON format.
///
/// # Examples
///
/// The following shows a sample retweet. To improve legibility, the retweet has been formatted into
/// multiple lines. In the actual file, single retweets must not contain any linebreaks.
///
/// ```json
/// {
///     "created_at": 987654321,
///     "text": "RT @ArthurDent: They say the Ultimate Answer is 42, how am I supposed to know
///              what the question is? Could be anything, I mean, what's 6x7?",
///     "id": 2,
///     "retweeted_status": {
///         "created_at": 123456789,
///         "text": "They say the Ultimate Answer is 42, how am I supposed to know what the
///                  question is? Could be anything, I mean, what's 6x7?",
///         "id": 1,
///         "user": {
///             "id": 42,
///             "screen_name": "ArthurDent"
///         },
///         "retweet_count": 1
///     },
///     "user": {
///         "id": 1337,
///         "screen_name": "ZaphodB"
///     },
///     "retweet_count": 1
/// }
/// ```
pub fn from_file<P>(filename: P) -> Vec<Tweet>
    where P: AsRef<Path> {
    let file = File::open(filename).expect("Could not open file.");
    let file = BufReader::new(file);

    let retweets: Vec<Tweet> = file.lines()
        .map(|line| serde_json::from_str(&line.expect("{}")).unwrap())
        .collect();

    retweets
}
