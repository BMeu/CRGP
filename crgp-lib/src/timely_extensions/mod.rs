// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! A collection of extension traits for ``timely``.
//!
//! # See Also
//! http://www.frankmcsherry.org/timely-dataflow/timely/index.html

pub use self::sync::Sync;

pub mod sync;
pub mod operators;
