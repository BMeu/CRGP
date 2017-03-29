extern crate crgplib;
#[cfg(unix)]
extern crate gag;

#[cfg(unix)]
use std::io::Read;
use std::sync::{Arc, Mutex};

#[cfg(unix)]
use gag::BufferRedirect;

use crgplib::{Result, Statistics};
use crgplib::algorithm;
use crgplib::social_graph::source::*;

#[test]
fn from_csv_files() {
    let batch_size: usize = 1;
    let print_result: bool = true;
    let friendship_dataset = SocialGraphCSVFiles::new("data/tests/friends");
    let friendships: Arc<Mutex<Option<SocialGraphCSVFiles>>> = Arc::new(Mutex::new(Some(friendship_dataset)));
    let retweet_dataset = String::from("data/tests/cascade.json");
    let timely_arguments = std::iter::empty::<String>();

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let mut buffer = BufferRedirect::stdout().unwrap();
        let result: Result<Statistics> = algorithm::execute(friendships, retweet_dataset, batch_size, print_result, timely_arguments);
        let mut output = String::new();
        buffer.read_to_string(&mut output).unwrap();
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        assert_eq!(influences.len(), 7);
        let expected_lines: Vec<&str> = vec![
            "Worker 0: 0 -> 2 at time 1 (cascade 1)",
            "Worker 0: 2 -> 1 at time 2 (cascade 1)",
            "Worker 0: 3 -> 1 at time 2 (cascade 1)",
            "Worker 0: 1 -> 0 at time 3 (cascade 2)",
            "Worker 0: 2 -> 3 at time 3 (cascade 1)",
            "Worker 0: 0 -> 2 at time 4 (cascade 2)",
            "Worker 0: 2 -> 3 at time 5 (cascade 2)"
        ];
        for expected_line in expected_lines {
            assert!(influences.contains(&expected_line));
        }
    }
        else {
            let result: Result<Statistics> = algorithm::execute(friendships, retweet_dataset, batch_size, print_result, timely_arguments);
            assert!(result.is_ok());
        }
}

#[test]
fn from_text_file() {
    let batch_size: usize = 1;
    let print_result: bool = true;
    let friendship_dataset = SocialGraphTextFile::new("data/tests/friends.txt");
    assert!(friendship_dataset.is_ok());

    let friendship_dataset: SocialGraphTextFile = friendship_dataset.unwrap();
    let friendships: Arc<Mutex<Option<SocialGraphTextFile>>> = Arc::new(Mutex::new(Some(friendship_dataset)));
    let retweet_dataset = String::from("data/tests/cascade.json");
    let timely_arguments = std::iter::empty::<String>();

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let mut buffer = BufferRedirect::stdout().unwrap();
        let result: Result<Statistics> = algorithm::execute(friendships, retweet_dataset, batch_size, print_result, timely_arguments);
        let mut output = String::new();
        buffer.read_to_string(&mut output).unwrap();
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        assert_eq!(influences.len(), 7);
        let expected_lines: Vec<&str> = vec![
            "Worker 0: 0 -> 2 at time 1 (cascade 1)",
            "Worker 0: 2 -> 1 at time 2 (cascade 1)",
            "Worker 0: 3 -> 1 at time 2 (cascade 1)",
            "Worker 0: 1 -> 0 at time 3 (cascade 2)",
            "Worker 0: 2 -> 3 at time 3 (cascade 1)",
            "Worker 0: 0 -> 2 at time 4 (cascade 2)",
            "Worker 0: 2 -> 3 at time 5 (cascade 2)"
        ];
        for expected_line in expected_lines {
            assert!(influences.contains(&expected_line));
        }
    }
    else {
        let result: Result<Statistics> = algorithm::execute(friendships, retweet_dataset, batch_size, print_result, timely_arguments);
        assert!(result.is_ok());
    }
}
