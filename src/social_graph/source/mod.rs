//! Load the social graph from various sources.

pub use self::text_file::parse_line;
pub use self::text_file::SocialGraphTextFile;

pub mod text_file;
