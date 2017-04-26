// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Collection of validator functions for the command-line arguments.

/// Ensure `value` is parsable to `usize`.
pub fn usize(value: String) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(_) => Ok(()),
        _ => Err(String::from("The value must be an integer."))
    }
}

/// Ensure `value` is parsable to `usize` with a value greater than `0`.
pub fn positive_usize(value: String) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(value) if value > 0 => Ok(()),
        _ => Err(String::from("The value must be a positive integer."))
    }
}
