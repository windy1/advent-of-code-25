use std::fmt::{self, Display, Formatter};

use log::debug;

const INPUT: &str = include_str!("../input.txt");
const POWERED_BATTERIES: usize = 12;

struct Bank(Vec<u32>);

impl Bank {
    fn find_max_joltage(&self) -> u64 {
        let data = &self.0;
        let mut digits = [0; POWERED_BATTERIES];
        let mut cursor: usize = 0;

        // For each digit slot
        for (i, slot) in digits.iter_mut().enumerate() {
            // Find the max of the eligible digits
            let num_remaining_digits = POWERED_BATTERIES - i;
            let max_window_size = data.len() - cursor;
            let window_size = max_window_size - num_remaining_digits + 1;

            let eligible_digits = &data[cursor..cursor + window_size];
            let mut candidate: Option<u32> = None;
            let mut candidate_index: Option<usize> = None;

            for (j, digit) in eligible_digits.iter().enumerate() {
                match candidate {
                    None => {
                        // Initialize candidate digit
                        candidate = Some(*digit);
                        candidate_index = Some(j + cursor);
                    }
                    Some(current) => {
                        if *digit > current {
                            // Found a higher eligible digit
                            candidate = Some(*digit);
                            candidate_index = Some(j + cursor);
                        }
                    }
                }
            }

            // Assign the candidate to the current digit slot
            *slot = candidate.expect("could not find digit");

            // Move the cursor past the new digit
            cursor = candidate_index.expect("could not find digit index") + 1;
        }

        digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str: String = self
            .0
            .iter()
            .map(|battery| char::from_digit(*battery, 10).expect("invalid digit"))
            .collect();
        write!(f, "{}", str)
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let banks: Vec<Bank> = INPUT.lines().map(collect_digits).map(Bank).collect();
    let mut joltage = 0;

    for bank in banks {
        let bank_joltage = bank.find_max_joltage();
        debug!("{} -> {}", bank, bank_joltage);
        joltage += bank_joltage;
    }

    println!("Joltage: {}", joltage);
}

fn collect_digits(str: &str) -> Vec<u32> {
    str.chars()
        .map(|c| c.to_digit(10).expect("invalid digit"))
        .collect::<Vec<u32>>()
}
