extern crate crgplib;

use std::sync::{Arc, Mutex};

use crgplib::{Result, Statistics};
use crgplib::algorithm;
use crgplib::social_graph::source::*;

#[test]
fn from_csv_files() {
    let batch_size: usize = 1;
    let print_result: bool = false;
    let friendship_dataset = SocialGraphCSVFiles::new("data/tests/friends");
    let friendships: Arc<Mutex<Option<SocialGraphCSVFiles>>> = Arc::new(Mutex::new(Some(friendship_dataset)));
    let retweet_dataset = String::from("data/tests/cascade.json");
    let timely_arguments = std::iter::empty::<String>();

    let result: Result<Statistics> = algorithm::execute(friendships, retweet_dataset, batch_size, print_result, timely_arguments);
    assert!(result.is_ok());
}

#[test]
fn from_text_file() {
    let batch_size: usize = 1;
    let print_result: bool = false;
    let friendship_dataset = SocialGraphTextFile::new("data/tests/friends.txt");
    assert!(friendship_dataset.is_ok());

    let friendship_dataset: SocialGraphTextFile = friendship_dataset.unwrap();
    let friendships: Arc<Mutex<Option<SocialGraphTextFile>>> = Arc::new(Mutex::new(Some(friendship_dataset)));
    let retweet_dataset = String::from("data/tests/cascade.json");
    let timely_arguments = std::iter::empty::<String>();

    let result: Result<Statistics> = algorithm::execute(friendships, retweet_dataset, batch_size, print_result, timely_arguments);
    assert!(result.is_ok());
}
