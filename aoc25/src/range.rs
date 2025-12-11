use std::ops;

const INVALID_FORMAT: &str = "invalid format";

pub struct Range(u64, u64);

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('-').expect(INVALID_FORMAT);
        let start = start.parse::<u64>().expect(INVALID_FORMAT);
        let end = end.parse::<u64>().expect(INVALID_FORMAT);
        Range(start, end)
    }
}

impl From<Range> for ops::Range<u64> {
    fn from(value: Range) -> Self {
        value.0..value.1
    }
}

impl From<Range> for ops::RangeInclusive<u64> {
    fn from(value: Range) -> Self {
        value.0..=value.1
    }
}
