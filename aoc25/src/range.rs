use std::{
    fmt::{self, Display, Formatter},
    ops,
};

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

impl From<ops::RangeInclusive<u64>> for Range {
    fn from(value: ops::RangeInclusive<u64>) -> Self {
        Range(*value.start(), *value.end())
    }
}

impl From<ops::Range<u64>> for Range {
    fn from(value: ops::Range<u64>) -> Self {
        Range(value.start, value.end)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

pub fn bidirectional_range(start: i64, delta: i64) -> Box<dyn Iterator<Item = i64>> {
    if delta >= 0 {
        Box::new(start..=start + delta)
    } else {
        Box::new((start + delta..=start).rev())
    }
}
