use std::ops::RangeInclusive;

use aoc25::range::Range;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let sections: Vec<&str> = INPUT.split("\n\n").collect();
    let ranges = parse_ranges(sections[0]);
    let ingredient_ids: Vec<u64> = parse_ingredient_ids(sections[1]);

    let num_fresh = ingredient_ids
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    println!("Answer: {}", num_fresh);
}

fn parse_ranges(s: &str) -> Vec<RangeInclusive<u64>> {
    s.split("\n")
        .map(Range::from)
        .map(RangeInclusive::from)
        .collect()
}

fn parse_ingredient_ids(s: &str) -> Vec<u64> {
    s.split("\n")
        .map(|id| id.parse::<u64>().expect("invalid ingredient ID"))
        .collect()
}
