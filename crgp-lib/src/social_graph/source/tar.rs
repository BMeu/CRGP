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
    let mut total_friendships: u64 = 0;
    let mut explicit_friendships: u64 = 0;
    let mut users: u64 = 0;

    // Top level.
    for root_entry in read_dir(path)? {
        let path: PathBuf = match root_entry {
            Ok(entry) => entry.path(),
            Err(_) => continue
        };

        // The entry must be a directory.
        if !path.is_dir() {
            continue;
        }

        // Get the last part of the path (e.g. `ZZZ` from `/xxx/yyy/ZZZ`).
        let path_c: PathBuf = path.clone();
        let directory: &str = match path_c.file_stem() {
            Some(directory) => {
                match directory.to_str() {
                    Some(directory) => directory,
                    None => continue
                }
            },
            None => continue
        };

        // Validate the name.
        if !DIRECTORY_NAME_TEMPLATE.is_match(directory) {
            trace!("Invalid directory name: {name:?}", name = path);
            continue;
        }

        // TAR archives.
        for archive_entry in read_dir(path)? {
            let path: PathBuf = match archive_entry {
                Ok(entry) => entry.path(),
                Err(_) => continue
            };

            // The entry must be a file.
            if !path.is_file() {
                continue;
            }

            // Get the file name.
            let path_c: PathBuf = path.clone();
            let filename: &str = match path_c.file_name() {
                Some(filename) => {
                    match filename.to_str() {
                        Some(filename) => filename,
                        None => continue
                    }
                },
                None => continue
            };

            // Validate the name.
            if !TAR_NAME_TEMPLATE.is_match(filename) {
                trace!("Invalid filename: {name:?}", name = path);
                continue;
            }

            // Open the archive.
            let archive_file = match File::open(path) {
                Ok(file) => file,
                Err(message) => {
                    error!("Could not open archive {archive:?}: {error}",
                    archive = path_c, error = message);
                    continue;
                }
            };
            let mut archive = Archive::new(archive_file);
            let archive_entries = match archive.entries() {
                Ok(entries) => entries,
                Err(message) => {
                    error!("Could not read contents of archive {archive:?}: {error}",
                    archive = path_c, error = message);
                    continue;
                }
            };
            for file in archive_entries {
                // Ensure correct reading.
                let file = match file {
                    Ok(file) => file,
                    Err(message) => {
                        error!("Could not read archived file in archive {archive:?}: {error}",
                        archive = path_c, error = message);
                        continue;
                    }
                };

                let path: PathBuf = match file.path() {
                    Ok(path) => path.to_path_buf(),
                    Err(_) => continue
                };
                // Validate the filename.
                match path.to_str() {
                    Some(path) => {
                        if !FILENAME_TEMPLATE.is_match(path) {
                            trace!("Invalid filename: {name:?}", name = path);
                            continue;
                        }
                    },
                    None => continue
                }

                // Get the user ID.
                let user: UserID = match path.file_stem() {
                    Some(stem) => {
                        match stem.to_str() {
                            Some(stem) => {
                                // `stem` is now `friends[ID]`. Only parse `[ID]`, i.e. skip the first seven
                                // characters.
                                match stem[7..].parse::<UserID>() {
                                    Ok(id) => id,
                                    Err(message) => {
                                        warn!("Could not parse user ID '{id}': {error}",
                                        id = &stem[7..], error = message);
                                        continue;
                                    }
                                }
                            },
                            None => continue
                        }
                    },
                    None => continue
                };

                // Parse the file.
                let reader = BufReader::new(file);
                let mut is_first_line: bool = true;
                let mut actual_number_of_friends: u64 = 0;
                let mut friendships: Vec<UserID> = reader.lines()
                    .filter_map(|line: IOResult<String>| -> Option<String> {
                        // Ensure correct encoding.
                        match line {
                            Ok(line) => Some(line),
                            Err(message) => {
                                warn!("Invalid line in file {file:?}: {error}", file = path, error = message);
                                None
                            }
                        }
                    })
                    .filter_map(|line: String| -> Option<UserID> {
                        // Parse the friend ID.
                        match line.parse::<UserID>() {
                            Ok(id) => Some(id),
                            Err(message) => {
                                // If this is the first line in the file, it may contain meta data.
                                let meta_data: Vec<&str> = if is_first_line {
                                    is_first_line = false;
                                    line.split(';')
                                        .collect()
                                } else {
                                    Vec::new()
                                };

                                // Index 3 contains the actual number of friendships.
                                if meta_data.len() == 5 {
                                    if let Ok(amount) = meta_data[3].parse() {
                                        actual_number_of_friends = amount;
                                        return None;
                                    }
                                }

                                warn!("Could not parse friend ID '{friend}' of user {user}: {error}",
                                friend = line, user = user, error = message);
                                None
                            }
                        }
                    })
                    .collect();

                let given_number_of_friends: u64 = friendships.len() as u64;
                trace!("User {user}: {given} of {actual} friends found", user = user,
                given = given_number_of_friends, actual = actual_number_of_friends);

                if given_number_of_friends > actual_number_of_friends {
                    warn!("User {user} has more friends ({given}) than claimed ({claim})", user = user,
                    given = given_number_of_friends, claim = actual_number_of_friends);
                }

                // Introduce dummy friends if required.
                let user_has_missing_friends: bool = given_number_of_friends < actual_number_of_friends;
                if pad_with_dummy_users && user_has_missing_friends {
                    // At this point, the given number of friends is guaranteed to be smaller than the actual
                    // number of friends.
                    let number_of_missing_friends: u64 = actual_number_of_friends - given_number_of_friends;
                    for dummy_id in 1..(number_of_missing_friends + 1) {
                        friendships.push(-(dummy_id as i64));
                    }
                    trace!("User {user}: created {number} dummy friends", user = user,
                    number = number_of_missing_friends);
                }

                // If the user still has no friends, continue.
                if friendships.is_empty() {
                    warn!("User {user} does not have any friends", user = user);
                    continue;
                }

                // Update social graph statistics.
                explicit_friendships += given_number_of_friends;
                total_friendships += actual_number_of_friends;
                users += 1;

                graph_input.send((user, friendships));
            }
        }
    }

    Ok((users, explicit_friendships, total_friendships))
}
