// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Extension traits for `Stream` implementing various custom `timely` operators.
//!
//! A collection of functions taking typed `Stream` objects from `timely` as input and producing new `Stream`
//! objects as output. These custom operators are specialized for the use in `CRGP`.

pub use self::find_possible_influences::FindPossibleInfluences;
pub use self::reconstruct::Reconstruct;
pub use self::write::Write;

mod find_possible_influences;
mod reconstruct;
mod write;
