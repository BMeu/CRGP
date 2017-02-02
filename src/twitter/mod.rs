//! Representations of data coming from Twitter and functions to work with those representations.

pub use self::tweet::Tweet;
pub use self::user::User;

pub mod load;
pub mod tweet;
pub mod user;
