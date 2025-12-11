use itertools::Itertools;
use log::debug;
use std::{collections::HashSet, ops::RangeInclusive};

const INPUT: &str = include_str!("../input.txt");

pub type RangeIn = RangeInclusive<u64>;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let sections: Vec<&str> = INPUT.split("\n\n").collect();
    let ranges = parse_ranges(sections[0]);
    let compressed = compress_ranges(&ranges);
    let answer = calc_fresh(&compressed);

    println!("Answer: {}", answer);
}

fn compress_ranges(ranges: &[RangeIn]) -> Vec<RangeIn> {
    let mut result: Vec<RangeIn> = vec![];

    for range in ranges {
        if result.is_empty() {
            result.push(range.clone());
            continue;
        }

        let last = result.last_mut().expect("should exist");

        if is_sub_range(range, last) {
            // This range is irrelevant
            debug!("Skipping {:?} is sub-range of {:?}", range, last);
            continue;
        }

        if is_sub_range(last, range) {
            // This new range contains the existing one
            debug!("Replacing {:?} is a sub-range of {:?}", last, range);
            *last = range.clone();
            continue;
        }

        if *range.start() <= *last.end() {
            // Extend the previous range
            debug!("Extending {:?} to {:?}", last, range);
            *last = *last.start()..=*range.end();
            continue;
        }

        // Add the discontiguous range
        debug!("Adding discontiguous range {:?}", range);
        result.push(range.clone());
    }

    result
}

fn calc_fresh(compressed: &[RangeIn]) -> u64 {
    let mut result = 0;
    for range in compressed {
        result += range.end() - range.start() + 1;
    }
    result
}

fn is_sub_range(little: &RangeIn, big: &RangeIn) -> bool {
    little.start() >= big.start() && little.end() <= big.end()
}

pub fn parse_ranges(s: &str) -> Vec<RangeIn> {
    s.split("\n")
        .collect::<HashSet<&str>>()
        .into_iter()
        .map(aoc25::range::Range::from)
        .map(RangeInclusive::from)
        .sorted_by_key(range_cmp_key)
        .collect()
}

pub fn range_cmp_key(range: &RangeIn) -> (u64, u64) {
    (*range.start(), *range.end())
}
