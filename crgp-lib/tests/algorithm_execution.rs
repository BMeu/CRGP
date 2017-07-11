// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

extern crate crgp_lib;
extern crate find_folder;
#[cfg(unix)]
extern crate gag;
#[cfg(unix)]
#[macro_use]
extern crate lazy_static;

#[cfg(unix)]
use std::io::Read;
use std::path::PathBuf;
use std::sync::Mutex;

use find_folder::Search;
#[cfg(unix)]
use gag::BufferRedirect;

use crgp_lib::Configuration;
use crgp_lib::Result;
use crgp_lib::Statistics;
use crgp_lib::configuration::Algorithm;
use crgp_lib::configuration::InputSource;

#[cfg(unix)]
lazy_static! {
    static ref STDOUT_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn algorithm_execution_gale() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .batch_size(1);

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}

#[test]
fn algorithm_execution_gale_with_selected_users() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());
    let selected_users: PathBuf = data_path.join("retweeting_users.txt");

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .batch_size(1)
        .selected_users(Some(selected_users));

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
    else {
        let result: Result<Statistics> = crgp_lib::run(configuration);
        assert!(result.is_ok());
    }
}

#[test]
fn algorithm_execution_gale_with_dummy_users() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .batch_size(1)
        .pad_with_dummy_users(true);

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}

#[test]
fn algorithm_execution_gale_with_selected_users_and_dummy_friends() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());
    let selected_users: PathBuf = data_path.join("retweeting_users.txt");

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .batch_size(1)
        .pad_with_dummy_users(true)
        .selected_users(Some(selected_users));

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}

#[test]
fn algorithm_execution_leaf() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .algorithm(Algorithm::LEAF)
        .batch_size(1);

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}

#[test]
fn algorithm_execution_leaf_with_selected_users() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());
    let selected_users: PathBuf = data_path.join("retweeting_users.txt");

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .algorithm(Algorithm::LEAF)
        .batch_size(1)
        .selected_users(Some(selected_users));

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}

#[test]
fn algorithm_execution_leaf_with_dummy_users() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .algorithm(Algorithm::LEAF)
        .batch_size(1)
        .pad_with_dummy_users(true);

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}

#[test]
fn algorithm_execution_leaf_with_selected_users_and_dummy_friends() {
    let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

    let friendship_dataset = InputSource::new(data_path.join("social_graph").to_str().unwrap());
    let retweet_dataset = InputSource::new(data_path.join("retweets.json").to_str().unwrap());
    let selected_users: PathBuf = data_path.join("retweeting_users.txt");

    let configuration = Configuration::default(retweet_dataset, friendship_dataset)
        .algorithm(Algorithm::LEAF)
        .batch_size(1)
        .pad_with_dummy_users(true)
        .selected_users(Some(selected_users));

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().expect("Could not lock STDOUT");
        let mut buffer = BufferRedirect::stdout().expect("Could not redirect STDOUT");
        let result: Result<Statistics> = crgp_lib::run(configuration);
        let mut output = String::new();
        buffer.read_to_string(&mut output).expect("Could not read STDOUT buffer");
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        let expected_lines: Vec<&str> = vec![
            "1;3;2;0;1;-1",
            "1;4;1;0;2;-1",
            "1;4;1;2;2;-1",
            "1;6;3;2;3;-1",
            "2;5;0;1;3;-1",
            "2;7;2;0;4;-1",
            "2;8;3;2;5;-1",
        ];
        for influence in &influences {
            assert!(expected_lines.contains(influence), "Unexpected influence: {}", influence);
        }
        for expected_line in &expected_lines {
            assert!(influences.contains(expected_line), "Missing influence: {}", expected_line);
        }
        assert_eq!(influences.len(), 7);
    }
        else {
            let result: Result<Statistics> = crgp_lib::run(configuration);
            assert!(result.is_ok());
        }
}
