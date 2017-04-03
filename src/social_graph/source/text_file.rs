//! Load the social graph from a text file.

use std::fs::File;
use std::io::{BufReader, Error, Lines};
use std::io::prelude::*;
use std::path::Path;

use social_graph::DirectedEdge;

/// A text file, on each line specifying a user followed by a list of all their friends.
///
/// Each user and friend is given by their user ID. The user is separated from their friends by a colon (`:`). The list of
/// friends is comma-separated (`,`). For example, if user `1` is friends with users `2` and `4`, the line would look
/// like this:
///
/// ```text
/// 1:2,4
/// ```
#[derive(Debug)]
pub struct SocialGraphTextFile {
    /// An iterator over the lines in the file.
    lines: Lines<BufReader<File>>,

    /// The user and a list of their friends currently being iterated over.
    current_user_and_friends: Option<(u64, Vec<u64>)>
}

impl SocialGraphTextFile {
    /// Open the given friends file. Returns a `std::io::Error` if there were problems opening the file.
    pub fn new<P>(filename: P) -> Result<SocialGraphTextFile, Error>
        where P: AsRef<Path> {
        let friendship_file = match File::open(&filename) {
            Ok(file) => file,
            Err(error) => {
                error!("Could not open friends file: {error}", error = error);
                return Err(Error::from(error));
            }
        };
        let reader: BufReader<File> = BufReader::new(friendship_file);
        let lines: Lines<BufReader<File>> = reader.lines();
        let mut file = SocialGraphTextFile { lines: lines, current_user_and_friends: None };
        file.set_current_user_and_friends();
        Ok(file)
    }

    /// Parse a single line of the friends file. The user ID is separated from the user's friends by a colon `:`, the
    /// friend IDs are comma-separated `,`. The list of friends will be in reversed order. If the given line is invalid
    /// `None` will be returned.
    ///
    /// The following cases invalidate a line:
    ///
    ///  * A user ID that is not parsable to `u64`.
    ///  * A user without friends.
    ///
    /// If a friend ID is not parsable to `u64`, it will be removed from the friends list but the line will still count
    /// as valid (unless by removing invalid friend IDs all of the user's friend are removed).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use crgplib::social_graph::source::SocialGraphTextFile;
    /// #
    /// assert_eq!(SocialGraphTextFile::parse_line(String::from("0:1,2")), Some((0, vec![2, 1])));
    /// assert_eq!(SocialGraphTextFile::parse_line(String::from("2:0")), Some((2, vec![0])));
    /// assert_eq!(SocialGraphTextFile::parse_line(String::from("4:a,2")), Some((4, vec![2])));
    ///
    /// assert_eq!(SocialGraphTextFile::parse_line(String::from("5:")), None);
    /// assert_eq!(SocialGraphTextFile::parse_line(String::from("a:1,2")), None);
    /// assert_eq!(SocialGraphTextFile::parse_line(String::from("6:a")), None);
    /// ```
    pub fn parse_line(line: String) -> Option<(u64, Vec<u64>)> {
        let user_and_friends: Vec<&str> = line.split(':').collect();

        // If the line is empty, it cannot be parsed.
        if user_and_friends.is_empty() {
            return None;
        }

        // Try to parse the user ID. If it fails, it does not make sense to parse the friends.
        let user: u64 = match user_and_friends[0].parse() {
            Ok(id) => id,
            Err(message) => {
                info!("Could not parse user ID '{id}': {error}", id = user_and_friends[0], error = message);
                return None
            }
        };

        // If the user has no friends, they can be skipped.
        let has_friends = user_and_friends.len() > 1 && !user_and_friends[1].is_empty();
        if !has_friends {
            warn!("User {id} has no valid friends", id = user);
            return None;
        }

        // Parse the friends.
        let mut friends: Vec<u64> = user_and_friends[1].split(',')
            .filter_map(|friend| {
                match friend.parse::<u64>() {
                    Ok(id) => Some(id),
                    Err(message) => {
                        info!("Could not parse friend ID '{friend}' of user {user}: {error}", friend = friend,
                              user = user, error = message);
                        None
                    }
                }
            })
            .collect();

        // If there are no friends left, the user can be skipped.
        if friends.is_empty() {
            warn!("User {id} has no valid friends", id = user);
            return None;
        }

        // Reverse the friends list so popping from it returns the elements in the correct order.
        friends.reverse();

        // Everything went fine!
        Some((user, friends))
    }

    /// Find the next valid line in the file (see `parse_line`) and set the current user and friends from it.
    fn set_current_user_and_friends(&mut self) {
        // Skip invalid lines.
        loop {
            // Get the next line. If there are no more lines, reset the current line to `None` and exit.
            let current_line: Result<String, Error> = match self.lines.next() {
                Some(line) => line,
                None => {
                    self.current_user_and_friends = None;
                    return;
                }
            };

            // If the line is ok, try to parse it. If that succeeds, exit. In all other cases continue.
            match current_line {
                Ok(line) => {
                    match SocialGraphTextFile::parse_line(line) {
                        Some(user_and_friends) => {
                            self.current_user_and_friends = Some(user_and_friends);
                            return;
                        },
                        None => {}
                    }
                },
                Err(_) => {
                    // TODO: Add `warn!()` with filename and line number about invalid UTF-8.
                }
            }
        }
    }
}

impl Iterator for SocialGraphTextFile {
    type Item = DirectedEdge<u64>;

    /// Iterate through all friend relations in the social graph.
    ///
    /// The results are returned in the order they appear in in the original file. However, the friends of a user will
    /// currently be returned in the reverse order.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get the current user and their friends. If there are none, the end of the iterator has been reached.
            let (user, mut friends): (u64, Vec<u64>) = match self.current_user_and_friends {
                Some((ref user, ref friends)) => (*user, friends.clone()),
                None => return None
            };

            match friends.pop() {
                Some(friend) => {
                    self.current_user_and_friends = Some((user, friends));
                    return Some(DirectedEdge::new(user, friend));
                },
                None => {
                    self.set_current_user_and_friends();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use social_graph::DirectedEdge;
    use super::*;

    #[test]
    fn new() {
        let file = SocialGraphTextFile::new("data/tests/friends.txt").unwrap();
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends, Some((0, vec![2, 1])));
    }

    #[test]
    fn parse_line() {
        assert_eq!(SocialGraphTextFile::parse_line(String::from("0:1,2")), Some((0, vec![2, 1])));
        assert_eq!(SocialGraphTextFile::parse_line(String::from("1:0,2,3")), Some((1, vec![3, 2, 0])));
        assert_eq!(SocialGraphTextFile::parse_line(String::from("2:0")), Some((2, vec![0])));
        assert_eq!(SocialGraphTextFile::parse_line(String::from("3:2")), Some((3, vec![2])));
        assert_eq!(SocialGraphTextFile::parse_line(String::from("a:1,2")), None);
        assert_eq!(SocialGraphTextFile::parse_line(String::from("4:a,2")), Some((4, vec![2])));
        assert_eq!(SocialGraphTextFile::parse_line(String::from("5:")), None);
        assert_eq!(SocialGraphTextFile::parse_line(String::from("6:a")), None);
    }

    #[test]
    fn set_current_user_and_friends() {
        let mut file = SocialGraphTextFile::new("data/tests/friends.txt").unwrap();
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends, Some((0, vec![2, 1])));

        file.set_current_user_and_friends();
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends, Some((1, vec![3, 2, 0])));

        file.set_current_user_and_friends();
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends, Some((2, vec![0])));

        file.set_current_user_and_friends();
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends, Some((3, vec![2])));

        file.set_current_user_and_friends();
        assert!(file.current_user_and_friends.is_some());
        assert_eq!(file.current_user_and_friends, Some((4, vec![2])));

        file.set_current_user_and_friends();
        assert!(file.current_user_and_friends.is_none());
    }

    #[test]
    fn next() {
        let mut file = SocialGraphTextFile::new("data/tests/friends.txt").unwrap();
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
