// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Measure the iteration performance of multiple structures.

#![feature(test)]

extern crate fnv;
extern crate crgp_lib;
extern crate fine_grained;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate tar;
extern crate test;

const SOCIAL_GRAPH_PATH: &str = "/home/bastian/Dokumente/data_sets/social_graph";
const SELECTED_USERS_PATH: &str = "benches/users.csv";

/// Measure the performance of sorted vectors.
mod vector_sorted {
    use fnv::FnvHashSet;
    use fine_grained::Stopwatch;
    use test::black_box;
    use test::Bencher;
    use test::stats::Summary;
    use super::fmt_thousands_sep;
    use super::SELECTED_USERS_PATH;
    use super::SOCIAL_GRAPH_PATH;
    use super::sg::SocialGraph;
    use super::sg::load;

    #[ignore]
    #[bench]
    fn iter_auto(bencher: &mut Bencher) {
        let mut sg: SocialGraph = SocialGraph::default();
        let _ = load(SOCIAL_GRAPH_PATH, true, Some(SELECTED_USERS_PATH), true, &mut sg).unwrap();

        let mut prefix: FnvHashSet<i64> = FnvHashSet::default();

        bencher.iter(|| {
            for (user, friends) in &sg {
                prefix.insert(*user);
                black_box(friends);
                for friend in friends {
                    black_box(prefix.contains(friend));
                }
            }
            prefix = FnvHashSet::default();
        });
    }

    #[ignore]
    #[bench]
    fn iter_10x(_bencher: &mut Bencher) {
        let mut sg: SocialGraph = SocialGraph::default();
        let _ = load(SOCIAL_GRAPH_PATH, true, Some(SELECTED_USERS_PATH), true, &mut sg).unwrap();

        let mut prefix: FnvHashSet<i64> = FnvHashSet::default();

        let mut stopwatch = Stopwatch::start_new();
        for _ in 0..10 {
            for (user, friends) in &sg {
                prefix.insert(*user);
                for friend in friends {
                    black_box(prefix.contains(friend));
                }
            }
            prefix = FnvHashSet::default();
            stopwatch.lap();
        }
        stopwatch.stop();

        let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
        let median: usize = summ.median as usize;
        let deviation: usize = (summ.max - summ.min) as usize;
        println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                           fmt_thousands_sep(deviation, ',')));
    }

    #[ignore]
    #[bench]
    fn iter_100x(_bencher: &mut Bencher) {
        let mut sg: SocialGraph = SocialGraph::default();
        let _ = load(SOCIAL_GRAPH_PATH, true, Some(SELECTED_USERS_PATH), true, &mut sg).unwrap();

        let mut prefix: FnvHashSet<i64> = FnvHashSet::default();

        let mut stopwatch = Stopwatch::start_new();
        for _ in 0..100 {
            for (user, friends) in &sg {
                prefix.insert(*user);
                for friend in friends {
                    black_box(prefix.contains(friend));
                }
            }
            prefix = FnvHashSet::default();
            stopwatch.lap();
        }
        stopwatch.stop();

        let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
        let median: usize = summ.median as usize;
        let deviation: usize = (summ.max - summ.min) as usize;
        println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                           fmt_thousands_sep(deviation, ',')));
    }

    /// Iterate only the users and all their friends.
    mod users_and_friends {
    }
}

/// Measure the performance of sorted vectors.
mod vector_unsorted {
    use fnv::FnvHashSet;
    use fine_grained::Stopwatch;
    use test::black_box;
    use test::Bencher;
    use test::stats::Summary;
    use super::fmt_thousands_sep;
    use super::SELECTED_USERS_PATH;
    use super::SOCIAL_GRAPH_PATH;
    use super::sg::SocialGraph;
    use super::sg::load;

    #[ignore]
    #[bench]
    fn iter_auto(bencher: &mut Bencher) {
        let mut sg: SocialGraph = SocialGraph::default();
        let _ = load(SOCIAL_GRAPH_PATH, true, Some(SELECTED_USERS_PATH), false, &mut sg).unwrap();

        let mut prefix: FnvHashSet<i64> = FnvHashSet::default();

        bencher.iter(|| {
            for (user, friends) in &sg {
                prefix.insert(*user);
                black_box(friends);
                for friend in friends {
                    black_box(prefix.contains(friend));
                }
            }
            prefix = FnvHashSet::default();
        });
    }

    #[ignore]
    #[bench]
    fn iter_10x(_bencher: &mut Bencher) {
        let mut sg: SocialGraph = SocialGraph::default();
        let _ = load(SOCIAL_GRAPH_PATH, true, Some(SELECTED_USERS_PATH), false, &mut sg).unwrap();

        let mut prefix: FnvHashSet<i64> = FnvHashSet::default();

        let mut stopwatch = Stopwatch::start_new();
        for _ in 0..10 {
            for (user, friends) in &sg {
                prefix.insert(*user);
                for friend in friends {
                    black_box(prefix.contains(friend));
                }
            }
            prefix = FnvHashSet::default();
            stopwatch.lap();
        }
        stopwatch.stop();

        let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
        let median: usize = summ.median as usize;
        let deviation: usize = (summ.max - summ.min) as usize;
        println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                           fmt_thousands_sep(deviation, ',')));
    }

    #[ignore]
    #[bench]
    fn iter_100x(_bencher: &mut Bencher) {
        let mut sg: SocialGraph = SocialGraph::default();
        let _ = load(SOCIAL_GRAPH_PATH, true, Some(SELECTED_USERS_PATH), false, &mut sg).unwrap();

        let mut prefix: FnvHashSet<i64> = FnvHashSet::default();

        let mut stopwatch = Stopwatch::start_new();
        for _ in 0..100 {
            for (user, friends) in &sg {
                prefix.insert(*user);
                for friend in friends {
                    black_box(prefix.contains(friend));
                }
            }
            prefix = FnvHashSet::default();
            stopwatch.lap();
        }
        stopwatch.stop();

        let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
        let median: usize = summ.median as usize;
        let deviation: usize = (summ.max - summ.min) as usize;
        println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                           fmt_thousands_sep(deviation, ',')));
    }
}

/// Format `n` with `sep`.
fn fmt_thousands_sep(mut n: usize, sep: char) -> String {
    use std::fmt::Write;
    let mut output = String::new();
    let mut trailing = false;
    for &pow in &[9, 6, 3, 0] {
        let base = 10_usize.pow(pow);
        if pow == 0 || trailing || n / base != 0 {
            if !trailing {
                output.write_fmt(format_args!("{}", n / base)).unwrap();
            } else {
                output.write_fmt(format_args!("{:03}", n / base)).unwrap();
            }
            if pow != 0 {
                output.push(sep);
            }
            trailing = true;
        }
        n %= base;
    }

    output
}

mod sg {
    use std::fs::read_dir;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Read;
    use std::io::Result as IOResult;
    use std::path::PathBuf;

    use fnv::FnvHashMap;
    use fnv::FnvHashSet;
    use regex::Regex;
    use tar::Archive;

    use crgp_lib::Result;

    pub type UserID = i64;
    pub type SocialGraph = FnvHashMap<UserID, Vec<UserID>>;

    lazy_static! {
        /// A regular expression to validate directory names. The name must consist of exactly three digits.
        // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
        // correct, it is safe to simply expect a valid result.
        #[derive(Debug)]
        static ref DIRECTORY_NAME_TEMPLATE: Regex = Regex::new(r"^\d{3}$").expect("Failed to compile the REGEX.");

        /// A regular expression to validate TAR file names. The name must consist of exactly two digits followed by the
        /// extension `.tar`.
        // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
        // correct, it is safe to simply expect a valid result.
        #[derive(Debug)]
        static ref TAR_NAME_TEMPLATE: Regex = Regex::new(r"^\d{2}\.tar$").expect("Failed to compile the REGEX.");

        /// A regular expression to validate file names. The name must be of the form `friends[ID].csv` where `[ID]`
        /// consists of one or more digits.
        // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
        // correct, it is safe to simply expect a valid result.
        #[derive(Debug)]
        static ref FILENAME_TEMPLATE: Regex = Regex::new(r"^\d{3}/\d{3}/friends\d+\.csv$")
            .expect("Failed to compile the REGEX.");
    }

    /// Load the social graph from the given `path` into the computation using the `graph_input`. If required, dummy users
    /// will be created. The function returns three counts in the following order: the number of users for whom friendships
    /// where loaded, the total number of explicitly given friendships, and the total number of all friendships.
    pub fn load(path: &str,
                pad_with_dummy_users: bool,
                selected_users_file: Option<&str>,
                sort: bool,
                graph_input: &mut SocialGraph
    ) -> Result<(u64, u64, u64)>
    {
        let path = PathBuf::from(String::from(path));

        // Get a set of selected users to load from the social graph. If `None`, the entire social graph will be loaded.
        let selected_users: Option<FnvHashSet<UserID>> = match selected_users_file {
            Some(file) => {
                let mut selected_users: FnvHashSet<UserID> = FnvHashSet::default();
                get_selected_friends(&PathBuf::from(String::from(file)), &mut selected_users)?;
                Some(selected_users)
            },
            None => None
        };

        let mut total_expected_friendships: u64 = 0;
        let mut total_given_friendships: u64 = 0;
        let mut users: u64 = 0;

        // Top level.
        for root_entry in read_dir(path)? {
            let directory_path: PathBuf = match root_entry {
                Ok(entry) => entry.path(),
                Err(_) => continue
            };

            if !is_valid_directory(&directory_path) {
                continue;
            }

            // TAR archives.
            for archive_entry in read_dir(directory_path)? {
                let tar_path: PathBuf = match archive_entry {
                    Ok(entry) => entry.path(),
                    Err(_) => continue
                };

                if !is_valid_tar_archive(&tar_path) {
                    continue;
                }

                // Open the archive and get its entries.
                let mut archive: Archive<File> = match File::open(tar_path.clone()) {
                    Ok(file) => Archive::new(file),
                    Err(_) => continue
                };
                let archive_entries = match archive.entries() {
                    Ok(entries) => entries,
                    Err(_) => continue
                };

                // Friend files.
                for file in archive_entries {
                    // Ensure correct reading.
                    let file = match file {
                        Ok(file) => file,
                        Err(_) => continue
                    };

                    let friends_path: PathBuf = match file.path() {
                        Ok(path) => path.to_path_buf(),
                        Err(_) => continue
                    };

                    if !is_valid_friend_file(&friends_path) {
                        continue;
                    }

                    // Get the user ID.
                    let user: UserID = match get_user_id(&friends_path) {
                        Some(id) => id,
                        None => continue
                    };

                    // If only selected users are requested: skip this user if they are not on the VIP list.
                    if let Some(ref selected_users) = selected_users {
                        if !selected_users.contains(&user) {
                            continue;
                        }
                    }

                    // Parse the file.
                    let reader = BufReader::new(file);
                    let (expected_friendships, mut friendships) = parse_friend_file(reader);
                    let given_friendships: u64 = friendships.len() as u64;

                    // Introduce dummy friends if required. To avoid any overflows, we must first ensure that there are less
                    // given friends than expected ones.
                    let user_has_missing_friends: bool = given_friendships < expected_friendships;
                    if pad_with_dummy_users && user_has_missing_friends {
                        let number_of_missing_friends: u64 = expected_friendships - given_friendships;
                        friendships.extend(create_dummy_friends(number_of_missing_friends));
                    }

                    // If the user still has no friends, continue.
                    if friendships.is_empty() {
                        continue;
                    }

                    // Update social graph statistics.
                    total_given_friendships += given_friendships;
                    total_expected_friendships += expected_friendships;
                    users += 1;

                    if sort {
                        friendships.sort();
                    }

                    graph_input.insert(user, friendships);
                }
            }
        }

        Ok((users, total_given_friendships, total_expected_friendships))
    }

    /// Load the given file `path` and insert all user IDs into the `out` set of friends to load. Errors on any I/O error.
    fn get_selected_friends(path: &PathBuf, out: &mut FnvHashSet<UserID>) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let id: String = match line {
                Ok(line) => line,
                Err(_) => continue
            };

            match id.parse::<UserID>() {
                Ok(id) => {
                    let _ = out.insert(id);
                },
                Err(_) => continue
            }
        }

        Ok(())
    }

    /// Create the given `amount` of dummy friends.
    fn create_dummy_friends(amount: u64) -> Vec<UserID> {
        let mut dummies: Vec<UserID> = Vec::new();
        for dummy_id in 1..(amount + 1) {
            dummies.push(-(dummy_id as UserID))
        }
        dummies
    }

    /// Get the user ID encoded in the file `path`. Return `None` if any error occurred.
    fn get_user_id(path: &PathBuf) -> Option<UserID> {
        if let Some(stem) = path.file_stem() {
            if let Some(stem) = stem.to_str() {
                match stem[7..].parse::<UserID>() {
                    Ok(id) => return Some(id),
                    Err(_) => return None,
                }
            }
        }

        None
    }

    /// Determine if the given path is a valid directory.
    fn is_valid_directory(path: &PathBuf) -> bool {
        if !path.is_dir() {
            return false;
        }

        if let Some(directory) = path.file_stem() {
            if let Some(directory) = directory.to_str() {
                if DIRECTORY_NAME_TEMPLATE.is_match(directory) {
                    return true;
                }
            }
        }

        false
    }

    /// Determine if the given path is a valid friend file.
    fn is_valid_friend_file(path: &PathBuf) -> bool {
        if let Some(filename) = path.to_str() {
            if FILENAME_TEMPLATE.is_match(filename) {
                return true;
            }
        }

        false
    }

    /// Determine if the given path is a valid tar archive.
    fn is_valid_tar_archive(path: &PathBuf) -> bool {
        if !path.is_file() {
            return false;
        }

        if let Some(filename) = path.file_name() {
            if let Some(filename) = filename.to_str() {
                if TAR_NAME_TEMPLATE.is_match(filename) {
                    return true;
                }
            }
        }

        false
    }

    /// Read the given friend file `reader` and parse its content. The parameters `file_path` and `user` are used in log
    /// messages for more detailed information on possible failures. Return the number of expected friends (i.e. as
    /// specified in the meta data) and a list of friends actually found in the file.
    fn parse_friend_file<R: Read>(reader: BufReader<R>) -> (u64, Vec<UserID>) {
        let mut is_first_line: bool = true;
        let mut expected_number_of_friends: u64 = 0;

        let found_friendships: Vec<UserID> = reader.lines()
            .filter_map(|line: IOResult<String>| -> Option<String> {
                // Ensure correct encoding.
                match line {
                    Ok(line) => Some(line),
                    Err(_) => None
                }
            })
            .filter_map(|line: String| -> Option<UserID> {
                // If this is the first line in the file, it may contain meta data.
                if is_first_line && line.contains(';') {
                    is_first_line = false;
                    if let Some(amount) = line.split(';').nth(3) {
                        if let Ok(amount) = amount.parse::<u64>() {
                            expected_number_of_friends = amount;
                        }
                    }

                    // The line cannot be a valid friend ID at this point anymore.
                    return None;
                }

                // Otherwise, parse the line as a friend ID.
                match line.parse::<UserID>() {
                    Ok(id) => Some(id),
                    Err(_) => None
                }
            })
            .collect();

        (expected_number_of_friends, found_friendships)
    }
}
