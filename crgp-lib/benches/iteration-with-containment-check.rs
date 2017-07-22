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

/// Get a list with elements from -100 to -1.
fn get_100_failing_elements() -> Vec<i64> {
    let mut list: Vec<i64> = Vec::new();
    for i in -100..0 {
        list.push(i);
    }
    list
}

/// Get a list with 100 elements, 10x from 0 to 9.
fn get_succeeding_elements() -> Vec<i64> {
    let mut list: Vec<i64> = Vec::new();
    for _ in 0..10 {
        for i in 0..10 {
            list.push(i);
        }
    }
    list
}

/// Measure the performance of hash sets.
mod hashset {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use super::get_set;
    use super::get_unsorted_list_of_size;
    use super::get_100_failing_elements;
    use super::get_succeeding_elements;

    /// Get an unsorted list of the given `size`, turn it into a hash set, and return it.
    fn get_hashset_of_size(size: i64) -> HashSet<i64> {
        let list: Vec<i64> = get_unsorted_list_of_size(size);
        HashSet::from_iter(list)
    }

    /// Do the containment check on a set with 100 entries not present in the list.
    mod fail {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_hashset_of_size;
        use super::get_100_failing_elements;

        #[bench]
        fn a_10(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(10);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn b_50(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(50);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn c_100(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn d_500(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn e_1000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(1_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_5000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(5_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_10000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(10_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn g_50000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(50_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn h_100000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: HashSet<i64> = get_hashset_of_size(100_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod succeed {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_hashset_of_size;
        use super::get_succeeding_elements;

        #[bench]
        fn a_10(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(10);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn b_50(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(50);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn c_100(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn d_500(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn e_1000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(1_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_5000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(5_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_10000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(10_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn g_50000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(50_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn h_100000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: HashSet<i64> = get_hashset_of_size(100_000);

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
    use super::get_100_failing_elements;
    use super::get_succeeding_elements;

    /// Get an unsorted list of the given `size`, sort and return it.
    fn get_sorted_list_of_size(size: i64) -> Vec<i64> {
        let mut list: Vec<i64> = get_unsorted_list_of_size(size);
        list.sort();
        list
    }

    /// Do the containment check on a set with 100 entries not present in the list.
    mod fail {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_sorted_list_of_size;
        use super::get_100_failing_elements;

        #[bench]
        fn a_10(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(10);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn b_50(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(50);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn c_100(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn d_500(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn e_1000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(1_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn f_5000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(5_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn f_10000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(10_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn g_50000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(50_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn h_100000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_sorted_list_of_size(100_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod succeed {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_sorted_list_of_size;
        use super::get_succeeding_elements;

        #[bench]
        fn a_10(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(10);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn b_50(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(50);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn c_100(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn d_500(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn e_1000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(1_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn f_5000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(5_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn f_10000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(10_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn g_50000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(50_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }

        #[bench]
        fn h_100000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_sorted_list_of_size(100_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.binary_search(item));
                }
            });
        }
    }
}

/// Measure the performance of unsorted vectors.
mod vector_unsorted {
    use super::get_set;
    use super::get_unsorted_list_of_size;
    use super::get_100_failing_elements;
    use super::get_succeeding_elements;


    /// Do the containment check on a set with 100 entries not present in the list.
    mod fail {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_unsorted_list_of_size;
        use super::get_100_failing_elements;

        #[bench]
        fn a_10(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(10);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn b_50(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(50);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn c_100(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn d_500(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn e_1000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(1_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_5000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(5_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_10000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(10_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn g_50000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(50_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn h_100000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_100_failing_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(100_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod succeed {
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use super::get_set;
        use super::get_unsorted_list_of_size;
        use super::get_succeeding_elements;

        #[bench]
        fn a_10(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(10);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn b_50(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(50);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn c_100(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(100);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn d_500(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(500);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn e_1000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(1_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_5000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(5_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn f_10000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(10_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn g_50000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(50_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }

        #[bench]
        fn h_100000(bencher: &mut Bencher) {
            let list: Vec<i64> = get_succeeding_elements();
            let set: Vec<i64> = get_unsorted_list_of_size(100_000);

            bencher.iter(|| {
                for item in &list {
                    black_box(set.contains(item));
                }
            });
        }
    }
}
