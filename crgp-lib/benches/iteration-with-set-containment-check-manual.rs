// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Measure the iteration performance of multiple structures with containment check on hash sets.

#![feature(test)]

extern crate fine_grained;
extern crate rand;
extern crate test;

use std::collections::HashSet;

use rand::Rng;
use rand::SeedableRng;
use rand::StdRng;

/// Number of iterations.
const ITERATIONS: usize = 1000;

/// Format `n` with `sep`.
fn fmt_thousands_sep(mut n: usize, sep: char) -> String {
    use std::fmt::Write;
    let mut output = String::new();
    let mut trailing = false;
    for &pow in &[9, 6, 3, 0] {
        let base = 10_usize.pow(pow);
        if pow == 0 || trailing || n / base != 0 {
            if !trailing {
                output.write_fmt(format_args!("{}", n / base)).unwrap();
            } else {
                output.write_fmt(format_args!("{:03}", n / base)).unwrap();
            }
            if pow != 0 {
                output.push(sep);
            }
            trailing = true;
        }
        n %= base;
    }

    output
}

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
    use super::fmt_thousands_sep;
    use super::get_set;
    use super::get_unsorted_list_of_size;
    use super::ITERATIONS;

    /// Get an unsorted list of the given `size`, turn it into a hash set, and return it.
    fn get_hashset_of_size(size: i64) -> HashSet<i64> {
        let list: Vec<i64> = get_unsorted_list_of_size(size);
        HashSet::from_iter(list)
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod list_size_matching {
        use fine_grained::Stopwatch;
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use test::stats::Summary;
        use super::fmt_thousands_sep;
        use super::get_set;
        use super::get_hashset_of_size;
        use super::ITERATIONS;

        #[bench]
        fn iter_100000_containment_check(_bencher: &mut Bencher) {
            let list: HashSet<i64> = get_hashset_of_size(100_000);
            let set: HashSet<i64> = get_set(0, 100_000);

            let mut stopwatch = Stopwatch::start_new();
            for _ in 0..ITERATIONS {
                for item in &list {
                    black_box(set.contains(item));
                }
                stopwatch.lap();
            }
            stopwatch.stop();

            let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
            let median: usize = summ.median as usize;
            let deviation: usize = (summ.max - summ.min) as usize;
            println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                               fmt_thousands_sep(deviation, ',')));
        }
    }
}

/// Measure the performance of sorted vectors.
mod vector_sorted {
    use super::fmt_thousands_sep;
    use super::get_set;
    use super::get_unsorted_list_of_size;
    use super::ITERATIONS;

    /// Get an unsorted list of the given `size`, sort and return it.
    fn get_sorted_list_of_size(size: i64) -> Vec<i64> {
        let mut list: Vec<i64> = get_unsorted_list_of_size(size);
        list.sort();
        list
    }

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod list_size_matching {
        use fine_grained::Stopwatch;
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use test::stats::Summary;
        use super::fmt_thousands_sep;
        use super::get_set;
        use super::get_sorted_list_of_size;
        use super::ITERATIONS;

        #[bench]
        fn iter_100000_containment_check(_bencher: &mut Bencher) {
            let list: Vec<i64> = get_sorted_list_of_size(100_000);
            let set: HashSet<i64> = get_set(0, 100_000);

            let mut stopwatch = Stopwatch::start_new();
            for _ in 0..ITERATIONS {
                for item in &list {
                    black_box(set.contains(item));
                }
                stopwatch.lap();
            }
            stopwatch.stop();

            let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
            let median: usize = summ.median as usize;
            let deviation: usize = (summ.max - summ.min) as usize;
            println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                               fmt_thousands_sep(deviation, ',')));
        }
    }
}

/// Measure the performance of unsorted vectors.
mod vector_unsorted {
    use super::fmt_thousands_sep;
    use super::get_set;
    use super::get_unsorted_list_of_size;
    use super::ITERATIONS;

    /// Do the containment check on a set of the same size as the list, with matching elements for all list elements.
    mod list_size_matching {
        use fine_grained::Stopwatch;
        use std::collections::HashSet;
        use test::black_box;
        use test::Bencher;
        use test::stats::Summary;
        use super::fmt_thousands_sep;
        use super::get_set;
        use super::get_unsorted_list_of_size;
        use super::ITERATIONS;

        #[bench]
        fn iter_100000_containment_check(_bencher: &mut Bencher) {
            let list: Vec<i64> = get_unsorted_list_of_size(100_000);
            let set: HashSet<i64> = get_set(0, 100_000);

            let mut stopwatch = Stopwatch::start_new();
            for _ in 0..ITERATIONS {
                for item in &list {
                    black_box(set.contains(item));
                }
                stopwatch.lap();
            }
            stopwatch.stop();

            let summ = Summary::new(stopwatch.laps().iter().map(|s| *s as f64).collect::<Vec<f64>>().as_slice());
            let median: usize = summ.median as usize;
            let deviation: usize = (summ.max - summ.min) as usize;
            println!("bench: {}", format_args!("{:>11} ns/iter (+/- {})", fmt_thousands_sep(median, ','),
                                               fmt_thousands_sep(deviation, ',')));
        }
    }
}
