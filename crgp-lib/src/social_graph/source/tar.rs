// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Load the social graph from TAR files.

use std::collections::HashSet;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Result as IOResult;
use std::path::PathBuf;

use regex::Regex;
use s3::bucket::Bucket;
use s3::error::ErrorKind as S3ErrorKind;
use s3::error::S3Error;
use s3::serde_types::ListBucketResult;
use tar::Archive;

use Error;
use Result;
use UserID;
use configuration::InputSource;
use reconstruction::algorithms::GraphHandle;
use twitter::User;

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

/// Load the social graph from the given `input` into the computation using the `graph_input`. If required, dummy users
/// will be created. The function returns three counts in the following order: the number of users for whom friendships
/// where loaded, the total number of explicitly given friendships, the total number of all friendships, and the total
/// number of dummy friends.
pub fn load(input: InputSource,
            pad_with_dummy_users: bool,
            selected_users_file: Option<PathBuf>,
            graph_input: &mut GraphHandle
    ) -> Result<(u64, u64, u64, u64)>
{
    let path = input.path.clone();
    match input.s3 {
        Some(s3_config) => {
            load_from_s3(&path, &s3_config.get_bucket()?, pad_with_dummy_users, selected_users_file, graph_input)
        },
        None => {
            load_locally(&PathBuf::from(path), pad_with_dummy_users, selected_users_file, graph_input)
        }
    }
}

/// Load the social graph from the given local `path`.
fn load_locally(path: &PathBuf,
                pad_with_dummy_users: bool,
                selected_users_file: Option<PathBuf>,
                graph_input: &mut GraphHandle
    ) -> Result<(u64, u64, u64, u64)>
{
    // Get a set of selected users to load from the social graph. If `None`, the entire social graph will be loaded.
    let selected_users: Option<HashSet<UserID>> = match selected_users_file {
        Some(file) => {
            let mut selected_users: HashSet<UserID> = HashSet::new();
            get_selected_friends(&file, &mut selected_users)?;
            Some(selected_users)
        },
        None => None
    };

    let mut total_expected_friendships: u64 = 0;
    let mut total_given_friendships: u64 = 0;
    let mut total_dummy_friendships: u64 = 0;
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
                    error!("Could not open archive {archive}: {error}", archive = tar_path.display(), error = message);
                    continue;
                }
            };
            let archive_entries = match archive.entries() {
                Ok(entries) => entries,
                Err(message) => {
                    error!("Could not read contents of archive {archive}: {error}",
                           archive = tar_path.display(), error = message);
                    continue;
                }
            };

            // Friend files.
            for file in archive_entries {
                // Ensure correct reading.
                let file = match file {
                    Ok(file) => file,
                    Err(message) => {
                        error!("Could not read archived file in archive {archive}: {error}",
                               archive = tar_path.display(), error = message);
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
                let user_id: UserID = match get_user_id(&friends_path) {
                    Some(id) => id,
                    None => continue
                };

                // If only selected users are requested: skip this user if they are not on the VIP list.
                if let Some(ref selected_users) = selected_users {
                    if !selected_users.contains(&user_id) {
                        continue;
                    }
                }

                // Parse the file.
                let reader = BufReader::new(file);
                let (expected_friendships, mut friendships) = parse_friend_file(reader, &friends_path, user_id);
                let user = User::new(user_id);
                let given_friendships: u64 = friendships.len() as u64;

                // Introduce dummy friends if required. To avoid any overflows, we must first ensure that there are less
                // given friends than expected ones.
                let user_has_missing_friends: bool = given_friendships < expected_friendships;
                let number_of_dummy_users: u64 = if pad_with_dummy_users && user_has_missing_friends {
                    let number_of_missing_friends: u64 = expected_friendships - given_friendships;
                    friendships.extend(create_dummy_friends(number_of_missing_friends));
                    trace!("User {user}: created {number} dummy friends",
                           user = user, number = number_of_missing_friends);
                    number_of_missing_friends
                } else {
                    0
                };

                // If the user still has no friends, continue.
                if friendships.is_empty() {
                    warn!("User {user} does not have any friends", user = user);
                    continue;
                }

                // Update social graph statistics.
                total_given_friendships += given_friendships;
                total_expected_friendships += expected_friendships;
                total_dummy_friendships += number_of_dummy_users;
                users += 1;

                graph_input.send((user, friendships));
            }
        }
    }

    Ok((users, total_given_friendships, total_expected_friendships, total_dummy_friendships))
}

/// Load the social graph from the given AWS S3 `bucket`.
fn load_from_s3(path: &str,
                bucket: &Bucket,
                pad_with_dummy_users: bool,
                selected_users_file: Option<PathBuf>,
                graph_input: &mut GraphHandle
    ) -> Result<(u64, u64, u64, u64)>
{
    // Get a set of selected users to load from the social graph. If `None`, the entire social graph will be loaded.
    let selected_users: Option<HashSet<UserID>> = match selected_users_file {
        Some(file) => {
            let mut selected_users: HashSet<UserID> = HashSet::new();
            get_selected_friends(&file, &mut selected_users)?;
            Some(selected_users)
        },
        None => None
    };

    let mut total_expected_friendships: u64 = 0;
    let mut total_given_friendships: u64 = 0;
    let mut total_dummy_friendships: u64 = 0;
    let mut users: u64 = 0;

    // Get all objects in the given path.
    let (list, code): (ListBucketResult, u32) = bucket.list(path, None)?;
    if code != 200 {
        let message: String = format!("Could not get contents of AWS S3 bucket \"{bucket} (region {region})\": \
                                       HTTP error {code}",
                                      bucket = bucket.name, region = bucket.region, code = code);
        error!("{}", message);
        return Err(Error::from(S3Error::from_kind(S3ErrorKind::Msg(message))));
    }

    // Load all TAR archives and parse them.
    for entry in list.contents {
        // Validate the file name.
        if !TAR_NAME_TEMPLATE.is_match(&entry.key) {
            trace!("Invalid filename: {name}", name = entry.key);
            continue;
        }

        // Load the actual file.
        let (contents, code): (Vec<u8>, u32) = bucket.get(&entry.key)?;
        if code != 200 {
            let message: String = format!("Could not get file \"{file}\" from AWS S3 bucket \"{bucket} (region \
                                           {region})\": HTTP error {code}",
                                          file = entry.key, bucket = bucket.name, region = bucket.region, code = code);
            error!("{}", message);
            return Err(Error::from(S3Error::from_kind(S3ErrorKind::Msg(message))));
        }

        // The array of `u8`s is just the archive we want to read.
        let mut archive: Archive<&[u8]> = Archive::new(&contents);
        let archive_entries = match archive.entries() {
            Ok(entries) => entries,
            Err(message) => {
                error!("Could not read contents of archive {archive}: {error}",
                        archive = entry.key, error = message);
                continue;
            }
        };

        // Open the friend files.
        for file in archive_entries {
            // Ensure correct reading.
            let file = match file {
                Ok(file) => file,
                Err(message) => {
                    error!("Could not read archived file in archive {archive}: {error}",
                            archive = entry.key, error = message);
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
            let user_id: UserID = match get_user_id(&friends_path) {
                Some(id) => id,
                None => continue
            };

            // If only selected users are requested: skip this user if they are not on the VIP list.
            if let Some(ref selected_users) = selected_users {
                if !selected_users.contains(&user_id) {
                    continue;
                }
            }

            // Parse the file.
            let reader = BufReader::new(file);
            let (expected_friendships, mut friendships) = parse_friend_file(reader, &friends_path, user_id);
            let user = User::new(user_id);
            let given_friendships: u64 = friendships.len() as u64;

            // Introduce dummy friends if required. To avoid any overflows, we must first ensure that there are less
            // given friends than expected ones.
            let user_has_missing_friends: bool = given_friendships < expected_friendships;
            let number_of_dummy_users: u64 = if pad_with_dummy_users && user_has_missing_friends {
                let number_of_missing_friends: u64 = expected_friendships - given_friendships;
                friendships.extend(create_dummy_friends(number_of_missing_friends));
                trace!("User {user}: created {number} dummy friends",
                       user = user, number = number_of_missing_friends);
                number_of_missing_friends
            } else {
                0
            };

            // If the user still has no friends, continue.
            if friendships.is_empty() {
                warn!("User {user} does not have any friends", user = user);
                continue;
            }

            // Update social graph statistics.
            total_given_friendships += given_friendships;
            total_expected_friendships += expected_friendships;
            total_dummy_friendships += number_of_dummy_users;
            users += 1;

            graph_input.send((user, friendships));
        }
    }

    Ok((users, total_given_friendships, total_expected_friendships, total_dummy_friendships))
}

/// Create the given `amount` of dummy friends.
fn create_dummy_friends(amount: u64) -> Vec<User> {
    let mut dummies: Vec<User> = Vec::new();
    for dummy_id in 1..(amount + 1) {
        let dummy = User::new(-(dummy_id as UserID));
        dummies.push(dummy);
    }
    dummies
}

/// Load the given file `path` and insert all user IDs into the `out` set of friends to load. Errors on any I/O error.
fn get_selected_friends(path: &PathBuf, out: &mut HashSet<UserID>) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let id: String = match line {
            Ok(line) => line,
            Err(message) => {
                warn!("Invalid line in file {file}: {error}", file = path.display(), error = message);
                continue;
            }
        };

        match id.parse::<UserID>() {
            Ok(id) => {
                let _ = out.insert(id);
            },
            Err(message) => {
                warn!("Could not parse user ID '{user}' in file {file}: {error}",
                      user = id, file = path.display(), error = message);
                continue;
            }
        }
    }

    Ok(())
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
            trace!("Invalid directory name: {name}", name = path.display());
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
        trace!("Invalid filename: {name}", name = path.display());
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
            trace!("Invalid filename: {name}", name = path.display());
        }
    }

    false
}

/// Read the given friend file `reader` and parse its content. The parameters `file_path` and `user` are used in log
/// messages for more detailed information on possible failures. Return the number of expected friends (i.e. as
/// specified in the meta data) and a list of friends actually found in the file.
fn parse_friend_file<R: Read>(reader: BufReader<R>, file_path: &PathBuf, user: UserID) -> (u64, Vec<User>) {
    let mut is_first_line: bool = true;
    let mut expected_number_of_friends: u64 = 0;

    let found_friendships: Vec<User> = reader.lines()
        .filter_map(|line: IOResult<String>| -> Option<String> {
            // Ensure correct encoding.
            match line {
                Ok(line) => Some(line),
                Err(message) => {
                    warn!("Invalid line in file {file}: {error}", file = file_path.display(), error = message);
                    None
                }
            }
        })
        .filter_map(|line: String| -> Option<User> {
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
            let id: UserID = match line.parse() {
                Ok(id) => id,
                Err(message) => {
                    warn!("Could not parse friend ID '{friend}' of user {user}: {error}",
                          friend = line, user = user, error = message);
                    return None;
                }
            };
            Some(User::new(id))
        })
        .collect();

    // Log how many friends were found.
    let given_friendships: u64 = found_friendships.len() as u64;
    trace!("User {user}: {given} of {expected} friends found",
           user = user, given = given_friendships, expected = expected_number_of_friends);

    // The data might be inconsistent and contain more friendships than expected.
    if given_friendships > expected_number_of_friends {
        warn!("User {user} has more friends ({given}) than claimed ({claim})",
              user = user, given = given_friendships, claim = expected_number_of_friends);
    }

    (expected_number_of_friends, found_friendships)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use find_folder::Search;
    use twitter::User;

    #[test]
    fn create_dummy_friends() {
        let dummy_friends: Vec<User> = super::create_dummy_friends(0);
        assert_eq!(dummy_friends.len(), 0);

        let dummy_friends: Vec<User> = super::create_dummy_friends(10);
        assert_eq!(dummy_friends.len(), 10);
        assert_eq!(dummy_friends[0], User::new(-1));
        assert_eq!(dummy_friends[1], User::new(-2));
        assert_eq!(dummy_friends[2], User::new(-3));
        assert_eq!(dummy_friends[3], User::new(-4));
        assert_eq!(dummy_friends[4], User::new(-5));
        assert_eq!(dummy_friends[5], User::new(-6));
        assert_eq!(dummy_friends[6], User::new(-7));
        assert_eq!(dummy_friends[7], User::new(-8));
        assert_eq!(dummy_friends[8], User::new(-9));
        assert_eq!(dummy_friends[9], User::new(-10));
    }

    #[test]
    fn get_user_id() {
        let valid = PathBuf::from(String::from("000/111/friends123.csv"));
        assert_eq!(super::get_user_id(&valid), Some(123));

        let valid = PathBuf::from(String::from("friends123.csv"));
        assert_eq!(super::get_user_id(&valid), Some(123));

        let invalid = PathBuf::from(String::from("000/111/friendsa.csv"));
        assert_eq!(super::get_user_id(&invalid), None);

        let invalid = PathBuf::from(String::from("friendsa.csv"));
        assert_eq!(super::get_user_id(&invalid), None);

        let invalid = PathBuf::from(String::from("000/111/friends.csv"));
        assert_eq!(super::get_user_id(&invalid), None);

        let invalid = PathBuf::from(String::from("friends.csv"));
        assert_eq!(super::get_user_id(&invalid), None);

        let invalid = PathBuf::from(String::from("000/111/friends"));
        assert_eq!(super::get_user_id(&invalid), None);

        let invalid = PathBuf::from(String::from("friends"));
        assert_eq!(super::get_user_id(&invalid), None);

        let invalid = PathBuf::from(String::from(".."));
        assert_eq!(super::get_user_id(&invalid), None);
    }

    #[test]
    fn is_valid_directory() {
        let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

        let valid: PathBuf = data_path.join("social_graph/000");
        assert!(super::is_valid_directory(&valid));

        let valid: PathBuf = data_path.join("social_graph/001");
        assert!(super::is_valid_directory(&valid));

        let invalid: PathBuf = data_path.join("social_graph");
        assert!(!super::is_valid_directory(&invalid));

        let invalid: PathBuf = data_path.join("social_graph/000/00.tar");
        assert!(!super::is_valid_directory(&invalid));
    }

    #[test]
    fn is_valid_friend_file() {
        let valid = PathBuf::from(String::from("000/111/friends123.csv"));
        assert!(super::is_valid_friend_file(&valid));

        let invalid = PathBuf::from(String::from("000"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/111"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("00/111/friends123.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("a/111/friends123.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/11/friends123.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/a/friends123.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/111/friend123.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/111/friends.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/111/friendsa.csv"));
        assert!(!super::is_valid_friend_file(&invalid));

        let invalid = PathBuf::from(String::from("000/111/friends123"));
        assert!(!super::is_valid_friend_file(&invalid));
    }

    #[test]
    fn is_valid_tar_archive() {
        let data_path: PathBuf = Search::ParentsThenKids(3, 3).for_folder("data").expect("Data folder not found.");

        let valid: PathBuf = data_path.join("social_graph/000/00.tar");
        assert!(super::is_valid_tar_archive(&valid));

        let valid: PathBuf = data_path.join("social_graph/001/00.tar");
        assert!(super::is_valid_tar_archive(&valid));

        let valid: PathBuf = data_path.join("social_graph/001/01.tar");
        assert!(super::is_valid_tar_archive(&valid));

        let invalid: PathBuf = data_path.join("social_graph/001/invalid.tar");
        assert!(!super::is_valid_tar_archive(&invalid));

        let invalid: PathBuf = data_path.join("social_graph/000");
        assert!(!super::is_valid_tar_archive(&invalid));
    }
}
