// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Load the social graph from multiple CSV files located in a defined directory structure.

use std::fs::{DirEntry, File, read_dir};
use std::io::{BufRead, BufReader, Error};
use std::path::{Path, PathBuf};

use log::LogLevel;
use regex::Regex;

use social_graph::DirectedEdge;

lazy_static! {
    /// A regular expression to validate directory names. The name must consist of exactly three digits.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    pub static ref DIRECTORY_NAME_TEMPLATE: Regex = Regex::new(r"^\d{3}$").unwrap();

    /// A regular expression to validate file names. The name must be of the form `friends[ID].csv` where `[ID]`
    /// consists of one or more digits.
    // The initialization of the Regex will fail if the expression is invalid. Since the expression is known to be
    // correct, it is safe to simply unwrap the result.
    #[derive(Debug)]
    pub static ref FILENAME_TEMPLATE: Regex = Regex::new(r"^friends\d+\.csv$").unwrap();
}

/// Multiple CSV files in a defined directory structure, each specifying all friends for a user.
///
/// Each CSV file contains all the friends (one per line) of a single user. The ID (`[ID]`, must be parsable to `u64`)
/// of a user is encoded into the filename and the directory path:
///
///  * Filename: `friends[ID].csv`
///  * Directory path: `[ID]` is padded with leading zeroes to twelve digits, then broken into a path with chunks of
///    size three.
///
/// # Examples
///
/// * The friends of user `42` are stored in `000/000/000/friends42.csv`.
/// * The friends of user `1337` are stored in `000/000/001/friends1337.csv`.
/// * The friends of user `420000000024` are stored in `420/000/000/friends420000000024.csv`.
#[derive(Debug)]
pub struct SocialGraphCSVFiles {
    /// `/XXX/___/___/friends_.csv`.
    top_level_directories: Vec<PathBuf>,

    /// `/xxx/YYY/___/friends_.csv`.
    second_level_directories: Vec<PathBuf>,

    /// `/xxx/yyy/ZZZ/friends_.csv`.
    third_level_directories: Vec<PathBuf>,

    /// /xxx/yyy/zzz/friendsA.csv`.
    friends_files_in_current_directory: Vec<PathBuf>,

    /// The user and an iterator over their friends currently being iterated over.
    current_user_and_friends: Option<(u64, Vec<u64>)>
}

impl SocialGraphCSVFiles {

    /// Initialize the CSV files and open the first file.
    ///
    /// The root directory (`root_directory`) is the directory that contains the structure explained above.
    pub fn new<P>(root_directory: P) -> SocialGraphCSVFiles
        where P: AsRef<Path> {
        let top_level_directories: Vec<PathBuf> = SocialGraphCSVFiles::get_valid_directories_in_path(root_directory);

        let mut file = SocialGraphCSVFiles {
            top_level_directories: top_level_directories,
            second_level_directories: vec![],
            third_level_directories: vec![],
            friends_files_in_current_directory: vec![],
            current_user_and_friends: None
        };
        file.set_current_user_and_friends();
        file
    }

    /// Get a vector of all valid directories.
    ///
    /// Get a vector of all directories in the given `path` that match the regular expression `DIRECTORY_NAME_TEMPLATE`
    /// defined in this module. The resulting vector is sorted in descending order on the directory names since the next
    /// element will be popped off from the end to maintain the expected ordering in the iterator. The result will be
    /// empty if there are no valid directories or any errors occurred during reading any of the involved paths.
    pub fn get_valid_directories_in_path<P>(path: P) -> Vec<PathBuf>
        where P: AsRef<Path> {
        let printable_path: PathBuf = path.as_ref().to_path_buf();
        let mut directories: Vec<PathBuf> = match read_dir(path) {
            Ok(directories) => {
                directories
                    .filter_map(|entry: Result<DirEntry, Error>| -> Option<PathBuf> {
                        if let Ok(entry) = entry {
                            // The entry must be a directory.
                            let path: PathBuf = entry.path();
                            if !path.is_dir() {
                                return None;
                            }

                            // Get the last part of the path (e.g. `ZZZ` from `/xxx/yyy/ZZZ`).
                            let path_c: PathBuf = path.clone();
                            let directory: &str = match path_c.file_stem() {
                                Some(directory) => {
                                    match directory.to_str() {
                                        Some(directory) => directory,
                                        None => return None
                                    }
                                },
                                None => return None
                            };

                            // Validate the name.
                            if DIRECTORY_NAME_TEMPLATE.is_match(directory) {
                                return Some(path);
                            }
                            trace!("Invalid directory name: {name}", name = directory)
                        }
                        None
                    })
                    .collect::<Vec<PathBuf>>()
            },
            Err(message) => {
                error!("Could not read directory {folder:?}: {error}", folder = printable_path, error = message);
                return Vec::<PathBuf>::new();
            }
        };

        // Sort the vector in reverse order because popping removes the last element.
        directories.sort_by(|a, b| b.cmp(a));
        directories
    }

    /// Get a vector of all valid files.
    ///
    /// Get a vector of all files in the given `path` that match the regular expression `FILENAME_TEMPLATE` defined in
    /// this module. The resulting vector is sorted in descending order on the filenames since the next element will be
    /// popped off from the end to maintain the expected ordering in the iterator. The result will be empty if there are
    /// no valid files or any errors occurred during reading any of the involved paths.
    pub fn get_valid_files_in_path<P>(path: P) -> Vec<PathBuf>
        where P: AsRef<Path> {
        let printable_path: PathBuf = path.as_ref().to_path_buf();
        let mut files: Vec<PathBuf> = match read_dir(path) {
            Ok(files) => {
                files
                    .filter_map(|entry: Result<DirEntry, Error>| -> Option<PathBuf> {
                        if let Ok(entry) = entry {
                            // The entry must be a file.
                            let path: PathBuf = entry.path();
                            if !path.is_file() {
                                return None;
                            }

                            // Get the last part of the path (e.g. `ZZZ` from `/xxx/yyy/ZZZ`).
                            let path_c: PathBuf = path.clone();
                            let filename: &str = match path_c.file_name() {
                                Some(filename) => {
                                    match filename.to_str() {
                                        Some(filename) => filename,
                                        None => return None
                                    }
                                },
                                None => return None
                            };

                            // Validate the name.
                            if FILENAME_TEMPLATE.is_match(filename) {
                                return Some(path);
                            }
                            trace!("Invalid filename: {name}", name = filename)
                        }
                        None
                    })
                    .collect::<Vec<PathBuf>>()
            },
            Err(message) => {
                error!("Could not read directory {folder:?}: {error}", folder = printable_path, error = message);
                return Vec::<PathBuf>::new();
            }
        };

        // Sort the vector in reverse order because popping removes the last element.
        files.sort_by(|a, b| b.cmp(a));
        files
    }

    /// Traverse through the top-level directories until one with valid second-level sub-directories is found. Set these
    /// as second-level directories and return `true`. If there are no more matching top-level directories, reset the
    /// second-level directories to empty and return `false`.
    fn set_second_level_directories(&mut self) -> bool {
        loop {
            let top_level: PathBuf = match self.top_level_directories.pop() {
                Some(top_level) => top_level,
                None => {
                    // If there are no more top-level directories, there is nothing else to do.
                    self.second_level_directories = vec![];
                    return false;
                }
            };

            let second_level_directories: Vec<PathBuf> = SocialGraphCSVFiles::get_valid_directories_in_path(top_level);
            if !second_level_directories.is_empty() {
                self.second_level_directories = second_level_directories;
                return true;
            }
        }
    }

    /// Traverse through the top and second-level directories until one with valid third-level sub-directories is
    /// found. Set these as third-level directories and return `true`. If there are no more matching top and
    /// second-level directories, reset the second and third-level directories to empty and return `false`.
    fn set_third_level_directories(&mut self) -> bool {
        loop {
            let second_level: PathBuf = match self.second_level_directories.pop() {
                Some(second_level) => second_level,
                None => {
                    // Try to set the next second-level directories and continue. If there are none, return `false`.
                    if !self.set_second_level_directories() {
                        self.third_level_directories = vec![];
                        return false;
                    }
                    continue;
                }
            };

            let third_level_directories: Vec<PathBuf> = SocialGraphCSVFiles::get_valid_directories_in_path(second_level);
            if !third_level_directories.is_empty() {
                self.third_level_directories = third_level_directories;
                return true;
            }
        }
    }

    /// Traverse through all directories from the root until one with valid friends files is found. Set these as the
    /// current friends files and return `true`. If there are no more matching directories, reset the directories and
    /// current friends files to empty and return `false`.
    fn set_friends_files_in_current_directory(&mut self) -> bool {
        loop {
            let third_level: PathBuf = match self.third_level_directories.pop() {
                Some(third_level) => third_level,
                None => {
                    // Try to set the next third-level directories and continue. If there are none, return `false`.
                    if !self.set_third_level_directories() {
                        self.friends_files_in_current_directory = vec![];
                        return false;
                    }
                    continue;
                }
            };

            let friends_files: Vec<PathBuf> = SocialGraphCSVFiles::get_valid_files_in_path(third_level);
            if !friends_files.is_empty() {
                self.friends_files_in_current_directory = friends_files;
                return true;
            }
        }
    }

    /// Find the next valid friends file in the defined directory structure and set the user and an iterator to their
    /// friends from it.
    ///
    /// The contents of the file are not checked for validity, e.g. the iterator might be empty from the start.
    fn set_current_user_and_friends(&mut self) {
        loop {
            let path: PathBuf = match self.friends_files_in_current_directory.pop() {
                Some(file) => file,
                None => {
                    // Try to set the next friends files and continue. If there are none, set the current user and their
                    // friends to `None` and return.
                    if !self.set_friends_files_in_current_directory() {
                        self.current_user_and_friends = None;
                        return;
                    }
                    continue;
                }
            };

            // Extract the user ID.
            let path_c: PathBuf = path.clone();
            let user: u64 = match path_c.file_stem() {
                Some(stem) => {
                    match stem.to_str() {
                        Some(stem) => {
                            // `stem` is now `friends[ID]`. Only parse `[ID]`, i.e. skip the first seven characters.
                            match stem[7..].parse::<u64>() {
                                Ok(id) => id,
                                Err(message) => {
                                    info!("Could not parse user ID '{id}': {error}", id = &stem[7..], error = message);
                                    continue;
                                }
                            }
                        },
                        None => continue
                    }
                },
                None => continue
            };

            // Open the file and parse all lines.
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(message) => {
                    error!("Could not open friends file {file:?}: {error}", file = path, error = message);
                    continue;
                }
            };
            let reader: BufReader<File> = BufReader::new(file);
            let mut friends: Vec<u64> = reader.lines()
                .filter_map(|line: Result<String, Error>| -> Option<u64> {
                    let line: String = match line {
                        Ok(line) => line,
                        Err(message) => {
                            warn!("Invalid line in file {file:?}: {error}", file = path, error = message);
                            return None;
                        }
                    };

                    let id: u64 = match line.parse() {
                        Ok(id) => id,
                        Err(message) => {
                            info!("Could not parse friend ID '{friend}' of user {user}: {error}", friend = line,
                                  user = user, error = message);
                            return None;
                        }
                    };
                    Some(id)
                })
                .collect();
            friends.reverse();

            if friends.is_empty() && log_enabled!(LogLevel::Info) {
                warn!("User {id} has no valid friends", id = user);
            }

            self.current_user_and_friends = Some((user, friends));
            return;
        }
    }
}

impl Iterator for SocialGraphCSVFiles {
    type Item = DirectedEdge<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get the current user and their friends. If there are none, the end of the iterator has been reached.
            let (user, mut friends): (u64, Vec<u64>) = match self.current_user_and_friends {
                Some((user, ref friends)) => (user, friends.clone()),
                None => return None
            };

            match friends.pop() {
                Some(friend) => {
                    self.current_user_and_friends = Some((user, friends));
                    return Some(DirectedEdge::new(user, friend));
                },
                None => self.set_current_user_and_friends()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn new() {
        let file = SocialGraphCSVFiles::new("data/tests/friends");
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends3.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends2.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends1.csv")
        ]);
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 0);
    }

    #[test]
    fn get_valid_directories_in_path() {
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends"), vec![
            PathBuf::from("data/tests/friends/001"),
            PathBuf::from("data/tests/friends/000")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/000"), vec![
            PathBuf::from("data/tests/friends/000/001"),
            PathBuf::from("data/tests/friends/000/000")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/000/000"), vec![
            PathBuf::from("data/tests/friends/000/000/001"),
            PathBuf::from("data/tests/friends/000/000/000")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/000/000/000"), Vec::<PathBuf>::new());
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/000/000/001"), Vec::<PathBuf>::new());
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/000/001"), vec![
            PathBuf::from("data/tests/friends/000/001/000")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/000/001/000"), Vec::<PathBuf>::new());
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/001"), vec![
            PathBuf::from("data/tests/friends/001/000")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/001/000"), vec![
            PathBuf::from("data/tests/friends/001/000/100")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends/001/000/100"), Vec::<PathBuf>::new());
    }

    #[test]
    fn get_valid_files_in_path() {
        assert_eq!(SocialGraphCSVFiles::get_valid_files_in_path("data/tests/friends"), Vec::<PathBuf>::new());
        assert_eq!(SocialGraphCSVFiles::get_valid_files_in_path("data/tests/friends/000/000/000"), vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends3.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends2.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends1.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends0.csv")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_files_in_path("data/tests/friends/000/000/001"), vec![
            PathBuf::from("data/tests/friends/000/000/001/friends1006.csv"),
            PathBuf::from("data/tests/friends/000/000/001/friends1005.csv")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_files_in_path("data/tests/friends/000/001/000"), Vec::<PathBuf>::new());
        assert_eq!(SocialGraphCSVFiles::get_valid_files_in_path("data/tests/friends/001/000/100"), vec![
            PathBuf::from("data/tests/friends/001/000/100/friends10001001.csv")
        ]);
        assert_eq!(SocialGraphCSVFiles::get_valid_files_in_path("data/tests/friends/001/01"), Vec::<PathBuf>::new());
    }

    #[test]
    fn set_second_level_directories() {
        let mut file = SocialGraphCSVFiles {
            top_level_directories: SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends"),
            second_level_directories: vec![],
            third_level_directories: vec![],
            friends_files_in_current_directory: vec![],
            current_user_and_friends: None
        };
        assert!(file.set_second_level_directories());
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001"),
            PathBuf::from("data/tests/friends/000/000")
        ]);
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());

        assert!(file.set_second_level_directories());
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/001/000")
        ]);
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());

        assert!(!file.set_second_level_directories());
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());
    }

    #[test]
    fn set_third_level_directories() {
        let mut file = SocialGraphCSVFiles {
            top_level_directories: SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends"),
            second_level_directories: vec![],
            third_level_directories: vec![],
            friends_files_in_current_directory: vec![],
            current_user_and_friends: None
        };
        assert!(file.set_third_level_directories());
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001"),
            PathBuf::from("data/tests/friends/000/000/000")
        ]);
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());

        assert!(file.set_third_level_directories());
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001/000")
        ]);
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());

        assert!(file.set_third_level_directories());
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/001/000/100")
        ]);
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());

        assert!(!file.set_third_level_directories());
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());
    }

    #[test]
    fn set_friends_files_in_current_directory() {
        let mut file = SocialGraphCSVFiles {
            top_level_directories: SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends"),
            second_level_directories: vec![],
            third_level_directories: vec![],
            friends_files_in_current_directory: vec![],
            current_user_and_friends: None
        };
        assert!(file.set_friends_files_in_current_directory());
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends3.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends2.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends1.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends0.csv")
        ]);
        assert!(file.current_user_and_friends.is_none());

        assert!(file.set_friends_files_in_current_directory());
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/001/friends1006.csv"),
            PathBuf::from("data/tests/friends/000/000/001/friends1005.csv")
        ]);
        assert!(file.current_user_and_friends.is_none());

        assert!(file.set_friends_files_in_current_directory());
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/001/000/100/friends10001001.csv")
        ]);
        assert!(file.current_user_and_friends.is_none());

        assert!(!file.set_friends_files_in_current_directory());
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());
    }

    #[test]
    fn set_current_user_and_friends() {
        let mut file = SocialGraphCSVFiles {
            top_level_directories: SocialGraphCSVFiles::get_valid_directories_in_path("data/tests/friends"),
            second_level_directories: vec![],
            third_level_directories: vec![],
            friends_files_in_current_directory: vec![],
            current_user_and_friends: None
        };
        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends3.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends2.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends1.csv")
        ]);
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 0);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, vec![2, 1]);

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends3.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends2.csv")
        ]);
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 1);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, vec![3, 2, 0]);

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv"),
            PathBuf::from("data/tests/friends/000/000/000/friends3.csv")
        ]);
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 2);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, vec![0]);

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/000/friends4.csv")
        ]);
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 3);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, vec![2]);

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, vec![
            PathBuf::from("data/tests/friends/000/000/001")
        ]);
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 4);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, vec![2]);

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, vec![
            PathBuf::from("data/tests/friends/000/000/001/friends1006.csv")
        ]);
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 1005);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, Vec::<u64>::new());

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, vec![
            PathBuf::from("data/tests/friends/001")
        ]);
        assert_eq!(file.second_level_directories, vec![
            PathBuf::from("data/tests/friends/000/001")
        ]);
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 1006);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, Vec::<u64>::new());

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().0, 10001001);
        assert_eq!(file.current_user_and_friends.as_ref().unwrap().1, Vec::<u64>::new());

        file.set_current_user_and_friends();
        assert_eq!(file.top_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.second_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.third_level_directories, Vec::<PathBuf>::new());
        assert_eq!(file.friends_files_in_current_directory, Vec::<PathBuf>::new());
        assert!(file.current_user_and_friends.is_none());
    }

    #[test]
    fn next() {
        let mut file = SocialGraphCSVFiles::new("data/tests/friends");
        assert_eq!(file.next(), Some(DirectedEdge::new(0, 1)));
        assert_eq!(file.next(), Some(DirectedEdge::new(0, 2)));
        assert_eq!(file.next(), Some(DirectedEdge::new(1, 0)));
        assert_eq!(file.next(), Some(DirectedEdge::new(1, 2)));
        assert_eq!(file.next(), Some(DirectedEdge::new(1, 3)));
        assert_eq!(file.next(), Some(DirectedEdge::new(2, 0)));
        assert_eq!(file.next(), Some(DirectedEdge::new(3, 2)));
        assert_eq!(file.next(), Some(DirectedEdge::new(4, 2)));
        assert_eq!(file.next(), None);
    }
}
