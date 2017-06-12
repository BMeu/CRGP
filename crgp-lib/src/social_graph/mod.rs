// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Traits for operating on a social graph.
//!
//! A social graph is a collection of directed edges.

pub use self::graph::SocialGraph;
pub use self::influence_edge::InfluenceEdge;

mod graph;
mod influence_edge;
pub mod source;
