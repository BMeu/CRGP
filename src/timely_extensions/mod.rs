//! A collection of extension traits for ``timely``.
//!
//! # See Also
//! http://www.frankmcsherry.org/timely-dataflow/timely/index.html

pub use self::sync::Sync;

pub mod sync;
pub mod operators;
