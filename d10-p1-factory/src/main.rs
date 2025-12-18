use std::fmt::{self, Display, Formatter};

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../input_example.txt");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.txt");
const ON: char = '#';
const OFF: char = '.';
const DIAGRAM_START: char = '[';
const DIAGRAM_END: char = ']';
const BUTTON_START: char = '(';
const BUTTON_END: char = ')';
const JOLTAGES_START: char = '{';
const JOLTAGES_END: char = '}';

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Machine {
    indicator_lights: IndicatorLights,
    desired_state: IndicatorLights,
    buttons: Vec<Button>,
    joltages: Joltages,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace();
        let mut desired_state: Option<IndicatorLights> = None;
        let mut buttons: Vec<Button> = vec![];
        let mut joltages: Option<Joltages> = None;

        for part in parts {
            if part.starts_with(DIAGRAM_START) {
                desired_state = Some(parse_input_part(part, &DIAGRAM_END, "diagram").into());
                continue;
            }

            if part.starts_with(BUTTON_START) {
                buttons.push(parse_input_part(part, &BUTTON_END, "button").into());
                continue;
            }

            if part.starts_with(JOLTAGES_START) {
                joltages = Some(parse_input_part(part, &JOLTAGES_END, "joltages").into());
            }
        }

        let desired_state = desired_state.expect("diagram not found");
        let joltages = joltages.expect("joltages not found");

        Self {
            indicator_lights: IndicatorLights(vec![false; desired_state.0.len()]),
            desired_state,
            buttons,
            joltages,
        }
    }
}

fn parse_input_part<'a>(part: &'a str, end: &char, kind: &str) -> &'a str {
    let end_pos = part
        .find(*end)
        .unwrap_or_else(|| panic!("missing closing bracket for {}", kind));
    &part[1..end_pos]
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.indicator_lights)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct IndicatorLights(Vec<bool>);

impl From<&str> for IndicatorLights {
    fn from(value: &str) -> Self {
        Self(value.chars().map(|ch| ch == ON).collect())
    }
}

impl Display for IndicatorLights {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for light in &self.0 {
            write!(f, "{}", if *light { ON } else { OFF })?;
        }

        write!(f, "]")
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Button(Vec<u32>);

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        Self(parse_u32_list(value))
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Joltages(Vec<u32>);

impl From<&str> for Joltages {
    fn from(value: &str) -> Self {
        Self(parse_u32_list(value))
    }
}

fn parse_u32_list(value: &str) -> Vec<u32> {
    value
        .split(",")
        .map(|value| value.parse().expect("invalid button digit"))
        .collect()
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let machines: Vec<_> = INPUT.lines().map(Machine::from).collect();

    println!("{:?}", machines);

    for machine in machines {
        println!("{}", machine);
    }
}
