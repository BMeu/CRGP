// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Measure the iteration performance of multiple structures.

#![feature(test)]

extern crate rand;
extern crate test;

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

/// Measure the performance of hash sets.
mod hashset {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use test::black_box;
    use test::Bencher;
    use super::get_unsorted_list_of_size;

    /// Get an unsorted list of the given `size`, turn it into a hash set, and return it.
    fn get_hashset_of_size(size: i64) -> HashSet<i64> {
        let list: Vec<i64> = get_unsorted_list_of_size(size);
        HashSet::from_iter(list)
    }

    #[bench]
    fn iter_10(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(10);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_50(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(50);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_100(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(100);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_500(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(500);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_1000(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(1_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_5000(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(5_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_10000(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(10_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_50000(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(50_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_100000(bencher: &mut Bencher) {
        let list: HashSet<i64> = get_hashset_of_size(100_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }
}

/// Measure the performance of sorted vectors.
mod vector_sorted {
    use test::black_box;
    use test::Bencher;
    use super::get_unsorted_list_of_size;

    /// Get an unsorted list of the given `size`, sort and return it.
    fn get_sorted_list_of_size(size: i64) -> Vec<i64> {
        let mut list: Vec<i64> = get_unsorted_list_of_size(size);
        list.sort();
        list
    }

    #[bench]
    fn iter_10(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(10);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_50(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(50);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_100(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(100);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_500(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(500);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_1000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(1_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_5000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(5_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_10000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(10_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_50000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(50_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_100000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_sorted_list_of_size(100_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }
}

/// Measure the performance of unsorted vectors.
mod vector_unsorted {
    use test::black_box;
    use test::Bencher;
    use super::get_unsorted_list_of_size;

    #[bench]
    fn iter_10(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(10);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_50(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(50);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_100(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(100);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_500(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(500);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_1000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(1_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_5000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(5_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_10000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(10_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_50000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(50_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }

    #[bench]
    fn iter_100000(bencher: &mut Bencher) {
        let list: Vec<i64> = get_unsorted_list_of_size(100_000);

        bencher.iter(|| {
            for item in &list {
                black_box(&item);
            }
        });
    }
}
