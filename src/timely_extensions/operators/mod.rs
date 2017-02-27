//! Extension traits for ``Stream`` implementing various custom ``timely`` operators.
//!
//! A collection of functions taking typed ``Stream`` objects from ``timely`` as input and producing
//! new ``Stream`` objects as output. These custom operators are specialized for the use in
//! ``CCGP``.

pub use self::find_possible_influences::FindPossibleInfluences;

pub mod find_possible_influences;
