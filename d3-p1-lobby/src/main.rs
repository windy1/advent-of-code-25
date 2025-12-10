use std::fmt::{self, Display, Formatter};

use log::debug;

const INPUT: &str = include_str!("../input.txt");

struct Bank(Vec<u32>);

impl Bank {
    fn find_max_joltage(&self) -> u32 {
        let data = &self.0;

        // Find the max digit excluding the final index
        let mut first_digit: Option<u32> = None;
        let mut first_digit_index: Option<usize> = None;

        for (i, digit) in data[..data.len() - 1].iter().enumerate() {
            match first_digit {
                None => {
                    first_digit = Some(*digit);
                    first_digit_index = Some(i);
                }
                Some(current) => {
                    if *digit > current {
                        first_digit = Some(*digit);
                        first_digit_index = Some(i);
                    }
                }
            }
        }

        let first_digit = first_digit.expect("could not find first digit");
        let first_digit_index = first_digit_index.expect("could not find first digit index");

        // Find the max digit to the right of the first digit
        let second_digit = data[first_digit_index + 1..]
            .iter()
            .max()
            .expect("could not find second digit");

        first_digit * 10 + second_digit
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
