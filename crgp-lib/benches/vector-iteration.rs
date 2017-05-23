// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Measure the iteration performance of sorted and unsorted `Vec`s in various sizes.

#![feature(test)]

extern crate rand;
extern crate test;

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

#[bench]
fn vec_iter_10_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_10_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_50_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_50_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_100_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_100_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_500_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(500);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_500_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(500);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_1000_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(1_000);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_1000_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(1_000);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_5000_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(5_000);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_5000_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(5_000);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_10000_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(10_000);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_10000_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(10_000);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_50000_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(50_000);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_50000_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(50_000);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_100000_sorted(bencher: &mut Bencher) {
    let mut list: Vec<i64> = get_unsorted_list_of_size(100_000);
    list.sort();

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}

#[bench]
fn vec_iter_100000_unsorted(bencher: &mut Bencher) {
    let list: Vec<i64> = get_unsorted_list_of_size(100_000);

    bencher.iter(|| {
        for item in list.iter() {
            black_box(&item);
        }
    });
}
