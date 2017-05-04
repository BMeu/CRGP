// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Load the social graph from TAR files.

use std::fs::read_dir;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Result as IOResult;
use std::path::PathBuf;

use regex::Regex;
use tar::Archive;
use timely::dataflow::operators::input::Handle;

use Result;
use UserID;

lazy_static! {
    /// A regular expression to validate directory names. The name must consist of exactly three digits.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    static ref DIRECTORY_NAME_TEMPLATE: Regex = Regex::new(r"^\d{3}$").unwrap();

    /// A regular expression to validate TAR file names. The name must consist of exactly two digits followed by the
    /// extension `.tar`.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    static ref TAR_NAME_TEMPLATE: Regex = Regex::new(r"^\d{2}\.tar$").unwrap();

    /// A regular expression to validate file names. The name must be of the form `friends[ID].csv` where `[ID]`
    /// consists of one or more digits.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    static ref FILENAME_TEMPLATE: Regex = Regex::new(r"^\d{3}/\d{3}/friends\d+\.csv$").unwrap();
}

/// Load the social graph from the given `path` into the computation using the `graph_input`. If required, dummy users
/// will be created. The function returns three counts in the following order: the number of users for whom friendships
/// where loaded, the total number of explicitly given friendships, and the total number of all friendships.
pub fn load(path: &PathBuf,
            pad_with_dummy_users: bool,
            graph_input: &mut Handle<u64, (UserID, Vec<UserID>)>
    ) -> Result<(u64, u64, u64)>
{
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
                Err(message) => {
                    error!("Could not open archive {archive:?}: {error}", archive = tar_path, error = message);
                    continue;
                }
            };
            let archive_entries = match archive.entries() {
                Ok(entries) => entries,
                Err(message) => {
                    error!("Could not read contents of archive {archive:?}: {error}",
                           archive = tar_path, error = message);
                    continue;
                }
            };

            // Friend files.
            for file in archive_entries {
                // Ensure correct reading.
                let file = match file {
                    Ok(file) => file,
                    Err(message) => {
                        error!("Could not read archived file in archive {archive:?}: {error}",
                               archive = tar_path, error = message);
                        continue;
                    }
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

                // Parse the file.
                let reader = BufReader::new(file);
                let (expected_friendships, mut friendships) = parse_friend_file(reader, &friends_path, user);

                // Log how many friends were found.
                let given_frienships: u64 = friendships.len() as u64;
                trace!("User {user}: {given} of {expected} friends found",
                       user = user, given = given_frienships, expected = expected_friendships);

                // The data might be inconsistent and contain more friendships than expected.
                if given_frienships > expected_friendships {
                    warn!("User {user} has more friends ({given}) than claimed ({claim})",
                          user = user, given = given_frienships, claim = expected_friendships);
                }

                // Introduce dummy friends if required. To avoid any overflows, we must first ensure that there are less
                // given friends than expected ones.
                let user_has_missing_friends: bool = given_frienships < expected_friendships;
                if pad_with_dummy_users && user_has_missing_friends {
                    let number_of_missing_friends: u64 = expected_friendships - given_frienships;
                    friendships.extend(create_dummy_friends(number_of_missing_friends));
                    trace!("User {user}: created {number} dummy friends",
                           user = user, number = number_of_missing_friends);
                }

                // If the user still has no friends, continue.
                if friendships.is_empty() {
                    warn!("User {user} does not have any friends", user = user);
                    continue;
                }

                // Update social graph statistics.
                total_given_friendships += given_frienships;
                total_expected_friendships += expected_friendships;
                users += 1;

                graph_input.send((user, friendships));
            }
        }
    }

    Ok((users, total_given_friendships, total_expected_friendships))
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
                Err(message) => {
                    warn!("Could not parse user ID '{id}': {error}", id = &stem[7..], error = message);
                    return None
                }
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
            trace!("Invalid directory name: {name:?}", name = path);
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
        trace!("Invalid filename: {name:?}", name = path);
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
            trace!("Invalid filename: {name:?}", name = path);
        }
    }

    false
}

/// Read the given friend file `reader` and parse its content. The parameters `file_path` and `user` are used in log
/// messages for more detailed information on possible failures. Return the number of expected friends (i.e. as
/// specified in the meta data) and a list of friends actually found in the file.
fn parse_friend_file<R: Read>(reader: BufReader<R>, file_path: &PathBuf, user: UserID) -> (u64, Vec<UserID>) {
    let mut is_first_line: bool = true;
    let mut expected_number_of_friends: u64 = 0;

    let found_friendships: Vec<UserID> = reader.lines()
        .filter_map(|line: IOResult<String>| -> Option<String> {
            // Ensure correct encoding.
            match line {
                Ok(line) => Some(line),
                Err(message) => {
                    warn!("Invalid line in file {file:?}: {error}", file = file_path, error = message);
                    None
                }
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
                Err(message) => {
                    warn!("Could not parse friend ID '{friend}' of user {user}: {error}",
                          friend = line, user = user, error = message);
                    None
                }
            }
        })
        .collect();

    (expected_number_of_friends, found_friendships)
}
