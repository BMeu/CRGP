//! Load the social graph from various sources.

pub use self::csv_files::SocialGraphCSVFiles;
pub use self::text_file::SocialGraphTextFile;

pub mod csv_files;
pub mod text_file;
