// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Measure the iteration performance of sorted and unsorted `Vec`s in various sizes with containment check on hash sets
//! for sets without any entries and sets with 100 non-matching entries.

#![feature(test)]

extern crate rand;
extern crate test;

use std::collections::HashSet;

use test::black_box;
use test::Bencher;

use rand::Rng;
use rand::SeedableRng;
use rand::StdRng;

/// Get an unsorted list of the given `size` of integers. Values are in the range `[0, size)`.
fn get_unsorted_list_of_size(size: i64) -> Vec<i64> {
    // Always use the same values.
    let seed: &[_] = &[0];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut list: Vec<i64> = Vec::new();
    for _ in 0..size {
        list.push(rng.gen_range(0, size));
    }
    list
}

/// Get a hash set with values in `[start, start + size)`.
fn get_set(start: i64, size: i64) -> HashSet<i64> {
    let mut set: HashSet<i64> = HashSet::new();
    for item in start..start + size {
        set.insert(item);
    }
    set
}

#[bench]
fn vec_iter_10_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_500_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(500);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_500_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(500);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_1000_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(1_000);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_1000_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(1_000);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_5000_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(5_000);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_5000_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(5_000);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10000_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10_000);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10000_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10_000);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50000_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50_000);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50000_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50_000);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100000_sorted_empty_set(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100_000);
    list.sort();
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100000_unsorted_empty_set(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100_000);
    let set: HashSet<i64> = HashSet::new();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10);
    list.sort();
    let set: HashSet<i64> = get_set(10, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10);
    let set: HashSet<i64> = get_set(10, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50);
    list.sort();
    let set: HashSet<i64> = get_set(50, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50);
    let set: HashSet<i64> = get_set(50, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100);
    list.sort();
    let set: HashSet<i64> = get_set(100, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100);
    let set: HashSet<i64> = get_set(100, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_500_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(500);
    list.sort();
    let set: HashSet<i64> = get_set(500, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_500_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(500);
    let set: HashSet<i64> = get_set(500, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_1000_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(1_000);
    list.sort();
    let set: HashSet<i64> = get_set(1_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_1000_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(1_000);
    let set: HashSet<i64> = get_set(1_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_5000_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(5_000);
    list.sort();
    let set: HashSet<i64> = get_set(5_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_5000_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(5_000);
    let set: HashSet<i64> = get_set(5_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10000_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10_000);
    list.sort();
    let set: HashSet<i64> = get_set(10_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10000_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10_000);
    let set: HashSet<i64> = get_set(10_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50000_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50_000);
    list.sort();
    let set: HashSet<i64> = get_set(50_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50000_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50_000);
    let set: HashSet<i64> = get_set(50_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100000_sorted_set_size_1(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100_000);
    list.sort();
    let set: HashSet<i64> = get_set(100_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100000_unsorted_set_size_1(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100_000);
    let set: HashSet<i64> = get_set(100_000, 1);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10);
    list.sort();
    let set: HashSet<i64> = get_set(10, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10);
    let set: HashSet<i64> = get_set(10, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50);
    list.sort();
    let set: HashSet<i64> = get_set(50, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50);
    let set: HashSet<i64> = get_set(50, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100);
    list.sort();
    let set: HashSet<i64> = get_set(100, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100);
    let set: HashSet<i64> = get_set(100, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_500_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(500);
    list.sort();
    let set: HashSet<i64> = get_set(500, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_500_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(500);
    let set: HashSet<i64> = get_set(500, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_1000_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(1_000);
    list.sort();
    let set: HashSet<i64> = get_set(1_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_1000_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(1_000);
    let set: HashSet<i64> = get_set(1_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_5000_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(5_000);
    list.sort();
    let set: HashSet<i64> = get_set(5_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_5000_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(5_000);
    let set: HashSet<i64> = get_set(5_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10000_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10_000);
    list.sort();
    let set: HashSet<i64> = get_set(10_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_10000_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10_000);
    let set: HashSet<i64> = get_set(10_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50000_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50_000);
    list.sort();
    let set: HashSet<i64> = get_set(50_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_50000_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50_000);
    let set: HashSet<i64> = get_set(50_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100000_sorted_set_size_100(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100_000);
    list.sort();
    let set: HashSet<i64> = get_set(100_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}

#[bench]
fn vec_iter_100000_unsorted_set_size_100(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100_000);
    let set: HashSet<i64> = get_set(100_000, 100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(set.contains(item));
        }
    });
}
