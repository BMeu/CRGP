// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

extern crate crgplib;
#[cfg(unix)]
extern crate gag;
#[cfg(unix)]
#[macro_use]
extern crate lazy_static;

#[cfg(unix)]
use std::io::Read;
use std::path::PathBuf;
use std::sync::Mutex;

#[cfg(unix)]
use gag::BufferRedirect;

use crgplib::{Result, Statistics};
use crgplib::algorithm;

#[cfg(unix)]
lazy_static! {
    static ref STDOUT_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn from_tar_archives() {
    let batch_size: usize = 1;
    let output_directory: Option<PathBuf> = None;
    let friendship_dataset = String::from("data/tests/friends-tar");
    let retweet_dataset = String::from("data/tests/cascade.json");
    let timely_arguments = std::iter::empty::<String>();

    // Capturing STDOUT currently only works on Unix systems.
    if cfg!(unix) {
        let _lock = STDOUT_MUTEX.lock().unwrap();
        let mut buffer = BufferRedirect::stdout().unwrap();
        let result: Result<Statistics> = algorithm::execute(friendship_dataset, retweet_dataset, batch_size, output_directory, timely_arguments);
        let mut output = String::new();
        buffer.read_to_string(&mut output).unwrap();
        drop(buffer);

        assert!(result.is_ok());
        let influences: Vec<&str> = output.split('\n')
            .filter(|line| !line.is_empty())
            .collect();
        assert_eq!(influences.len(), 7);
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
    }
        else {
            let result: Result<Statistics> = algorithm::execute(friendship_dataset, retweet_dataset, batch_size, output_directory, timely_arguments);
            assert!(result.is_ok());
        }
}
