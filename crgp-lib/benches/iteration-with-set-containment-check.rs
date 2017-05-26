// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Measure the iteration performance of multiple structures with containment check on hash sets.

#![feature(test)]

extern crate rand;
extern crate test;

use std::collections::HashSet;

use rand::Rng;
use rand::SeedableRng;
use rand::StdRng;

/// Get a hash set with values in `[start, start + size)`.
fn get_set(start: i64, size: i64) -> HashSet<i64> {
    let mut set: HashSet<i64> = HashSet::new();
    for item in start..start + size {
        set.insert(item);
    }
    set
}

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
    use super::get_set;
    use super::get_unsorted_list_of_size;

    /// Get an unsorted list of the given `size`, turn it into a hash set, and return it.
    fn get_hashset_of_size(size: i64) -> HashSet<i64> {
        let list: Vec<i64> = get_unsorted_list_of_size(size);
        let set: HashSet<i64> = HashSet::from_iter(list);
        set
    }

    /// Do the containment check on an empty set.
    mod empty_set {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_hashset_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(10);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(50);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(500);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(1_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(5_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(10_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(50_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set with 100 entries not present in the list.
    mod size_100_non_matching {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_hashset_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(10);
            let set: HashSet<i64> = get_set(10, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(50);
            let set: HashSet<i64> = get_set(50, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100);
            let set: HashSet<i64> = get_set(100, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(500);
            let set: HashSet<i64> = get_set(500, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(1_000);
            let set: HashSet<i64> = get_set(1_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(5_000);
            let set: HashSet<i64> = get_set(5_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(10_000);
            let set: HashSet<i64> = get_set(10_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(50_000);
            let set: HashSet<i64> = get_set(50_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100_000);
            let set: HashSet<i64> = get_set(100_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod list_size_matching {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_hashset_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(10);
            let set: HashSet<i64> = get_set(0, 10);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(50);
            let set: HashSet<i64> = get_set(0, 50);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100);
            let set: HashSet<i64> = get_set(0, 100);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(500);
            let set: HashSet<i64> = get_set(0, 500);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(1_000);
            let set: HashSet<i64> = get_set(0, 1_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(5_000);
            let set: HashSet<i64> = get_set(0, 5_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(10_000);
            let set: HashSet<i64> = get_set(0, 10_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(50_000);
            let set: HashSet<i64> = get_set(0, 50_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100_000);
            let set: HashSet<i64> = get_set(0, 100_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }
}

/// Measure the performance of sorted vectors.
mod vector_sorted {
    use super::get_set;
    use super::get_unsorted_list_of_size;

    /// Get an unsorted list of the given `size`, sort and return it.
    fn get_sorted_list_of_size(size: i64) -> Vec<i64> {
        let mut list: Vec<i64> = get_unsorted_list_of_size(size);
        list.sort();
        list
    }

    /// Do the containment check on an empty set.
    mod empty_set {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_sorted_list_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(10);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(50);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(500);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(1_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(5_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(10_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(50_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set with 100 entries not present in the list.
    mod size_100_non_matching {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_sorted_list_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(10);
            let set: HashSet<i64> = get_set(10, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(50);
            let set: HashSet<i64> = get_set(50, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100);
            let set: HashSet<i64> = get_set(100, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(500);
            let set: HashSet<i64> = get_set(500, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(1_000);
            let set: HashSet<i64> = get_set(1_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(5_000);
            let set: HashSet<i64> = get_set(5_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(10_000);
            let set: HashSet<i64> = get_set(10_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(50_000);
            let set: HashSet<i64> = get_set(50_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100_000);
            let set: HashSet<i64> = get_set(100_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod list_size_matching {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_sorted_list_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(10);
            let set: HashSet<i64> = get_set(0, 10);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(50);
            let set: HashSet<i64> = get_set(0, 50);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100);
            let set: HashSet<i64> = get_set(0, 100);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(500);
            let set: HashSet<i64> = get_set(0, 500);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(1_000);
            let set: HashSet<i64> = get_set(0, 1_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(5_000);
            let set: HashSet<i64> = get_set(0, 5_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(10_000);
            let set: HashSet<i64> = get_set(0, 10_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(50_000);
            let set: HashSet<i64> = get_set(0, 50_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100_000);
            let set: HashSet<i64> = get_set(0, 100_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }
}

/// Measure the performance of unsorted vectors.
mod vector_unsorted {
    use super::get_set;
    use super::get_unsorted_list_of_size;

    /// Do the containment check on an empty set.
    mod empty_set {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_unsorted_list_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(10);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(50);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(&item);
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(1_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(5_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(10_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(50_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100_000);
            let set: HashSet<i64> = HashSet::new();

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set with 100 entries not present in the list.
    mod size_100_non_matching {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_unsorted_list_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(10);
            let set: HashSet<i64> = get_set(10, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(50);
            let set: HashSet<i64> = get_set(50, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100);
            let set: HashSet<i64> = get_set(100, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(500);
            let set: HashSet<i64> = get_set(500, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(1_000);
            let set: HashSet<i64> = get_set(1_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(5_000);
            let set: HashSet<i64> = get_set(5_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(10_000);
            let set: HashSet<i64> = get_set(10_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(50_000);
            let set: HashSet<i64> = get_set(50_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100_000);
            let set: HashSet<i64> = get_set(100_000, 100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod list_size_matching {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_unsorted_list_of_size;

        #[bench]
        fn iter_10_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(10);
            let set: HashSet<i64> = get_set(0, 10);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(50);
            let set: HashSet<i64> = get_set(0, 50);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100);
            let set: HashSet<i64> = get_set(0, 100);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_500_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(500);
            let set: HashSet<i64> = get_set(0, 500);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_1000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(1_000);
            let set: HashSet<i64> = get_set(0, 1_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_5000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(5_000);
            let set: HashSet<i64> = get_set(0, 5_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_10000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(10_000);
            let set: HashSet<i64> = get_set(0, 10_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_50000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(50_000);
            let set: HashSet<i64> = get_set(0, 50_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn iter_100000_containment_check(bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100_000);
            let set: HashSet<i64> = get_set(0, 100_000);;

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }
}
