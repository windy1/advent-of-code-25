use log::debug;

const INPUT: &str = include_str!("../input.txt");
const INVALID_FORMAT: &str = "invalid format";

type Range = (u64, u64);

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let ranges: Vec<(u64, u64)> = INPUT.split(',').map(parse_range).collect();
    let mut answer: u64 = 0;

    for range in ranges {
        debug!("{}-{}", range.0, range.1);
        for id in range.0..=range.1 {
            if !is_valid_id(id) {
                debug!(" Invalid: {}", id);
                answer += id;
            }
        }
    }

    println!("Answer: {}", answer);
}

fn parse_range(value: &str) -> Range {
    let (start, end) = value.split_once('-').expect(INVALID_FORMAT);
    let start = start.parse::<u64>().expect(INVALID_FORMAT);    
    let end = end.parse::<u64>().expect(INVALID_FORMAT);
    (start, end)
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
            if remaining_digits.is_multiple_of(sequence.len()) {
                // We can fit one or more of the sequence in the remaining digits
                let repeated_sequence = sequence.repeat(remaining_digits / sequence.len());
                if id_str[i..] == repeated_sequence {
                    // The rest of the string repeats the current sequence
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
        assert!(!is_valid_id(111));
    }
}
