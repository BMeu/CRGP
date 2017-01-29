extern crate fnv;

use std::hash::*;

/// Hash an item.
pub fn hash<T: Hash>(item: &T) -> u64 {
    let mut h: fnv::FnvHasher = Default::default();
    item.hash(&mut h);
    h.finish()
}
