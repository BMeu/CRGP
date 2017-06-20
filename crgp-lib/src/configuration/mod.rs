// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(feature = "cargo-clippy", allow(stutter))]

//! Algorithm configuration.

pub use self::algorithm::Algorithm;
pub use self::input::InputSource;
pub use self::main::Configuration;
pub use self::output::OutputTarget;
pub use self::s3::S3;

mod algorithm;
mod input;
mod main;
mod output;
mod s3;
