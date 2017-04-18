//! Extension traits for ``Stream`` implementing various custom ``timely`` operators.
//!
//! A collection of functions taking typed ``Stream`` objects from ``timely`` as input and producing new ``Stream``
//! objects as output. These custom operators are specialized for the use in ``CRGP``.

pub use self::reconstruct::Reconstruct;
pub use self::write::Write;

pub mod reconstruct;
pub mod write;
