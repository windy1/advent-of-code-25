use std::ops::RangeInclusive;

use aoc25::range::Range;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let ranges = parse_ranges();
    let mut answer: u64 = 0;

    for range in ranges {
        for id in range {
            if !is_valid_id(id) {
                answer += id;
            }
        }
    }

    println!("Answer: {}", answer);
}

fn parse_ranges() -> Vec<RangeInclusive<u64>> {
    INPUT
        .split(',')
        .map(Range::from)
        .map(RangeInclusive::from)
        .collect()
}

fn is_valid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let num_digits = id_str.len();
    let digits = id_str.chars();
    let mut sequence = String::new();

    for (i, digit) in digits.enumerate() {
        if !sequence.is_empty() {
            // Check current sequence
            let remaining_digits = num_digits - i;
            if remaining_digits == sequence.len() {
                // We can fit a second repeat of the sequence in the remaining digits
                if id_str[i..] == sequence {
                    // The sequence repeats a second time
                    return false;
                }
            }
        }
        sequence.push(digit);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_id_repeated_sequence() {
        assert!(!is_valid_id(11));
        assert!(!is_valid_id(1212));
        assert!(!is_valid_id(123123));
        assert!(!is_valid_id(1188511885));
        assert!(is_valid_id(111));
    }
}
