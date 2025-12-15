use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use aoc25::math::{self, Point3i64};
use itertools::Itertools;
use log::debug;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../input_example.txt");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.txt");
#[cfg(debug_assertions)]
const N: usize = 10;
#[cfg(not(debug_assertions))]
const N: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct JunctionBox(usize, usize, usize);

impl From<&str> for JunctionBox {
    fn from(value: &str) -> Self {
        let coords: Vec<usize> = value
            .trim()
            .split(',')
            .map(|coord| coord.parse::<usize>().expect("invalid coordinate"))
            .collect();
        Self(coords[0], coords[1], coords[2])
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Circuit {
    junction_boxes: Vec<JunctionBox>,
}

impl From<Vec<JunctionBox>> for Circuit {
    fn from(junction_boxes: Vec<JunctionBox>) -> Self {
        Self { junction_boxes }
    }
}

impl From<JunctionBox> for Circuit {
    fn from(value: JunctionBox) -> Self {
        Self::from(vec![value])
    }
}

impl From<&JunctionBox> for Point3i64 {
    fn from(value: &JunctionBox) -> Self {
        (value.0 as i64, value.1 as i64, value.2 as i64)
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let junction_boxes: String = self
            .junction_boxes
            .iter()
            .map(JunctionBox::to_string)
            .join(", ");
        write!(f, "[{}]", junction_boxes)
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let junction_boxes: Vec<JunctionBox> = INPUT.lines().map(JunctionBox::from).collect();
    let distances = find_closest_junction_boxes(&junction_boxes);

    for (i, j, distance) in &distances {
        let value_a = &junction_boxes[*i];
        let value_b = &junction_boxes[*j];
        debug!("{} is {} away from {}", value_a, distance, value_b);
    }

    let mut circuits: Vec<Circuit> = vec![];
    let mut circuit_map: HashMap<JunctionBox, usize> = HashMap::default();

    for (i, j, distance) in distances.iter().take(N) {
        let value_a = &junction_boxes[*i];
        let value_b = &junction_boxes[*j];

        debug!("{} is {} away from {}", value_a, distance, value_b);

        if circuits.is_empty() {
            circuits.push(vec![*value_a, *value_b].into());
            circuit_map.insert(*value_a, 0);
            circuit_map.insert(*value_b, 0);
            debug_circuits(&circuits);
            continue;
        }

        let circuit_a = circuit_map.get(value_a).copied();
        let circuit_b = circuit_map.get(value_b).copied();

        match (circuit_a, circuit_b) {
            (None, None) => {
                circuits.push(vec![*value_a, *value_b].into());
                let new_index = circuits.len() - 1;
                circuit_map.insert(*value_a, new_index);
                circuit_map.insert(*value_b, new_index);
            }
            (None, Some(circuit_b)) => {
                circuits[circuit_b].junction_boxes.push(*value_a);
                circuit_map.insert(*value_a, circuit_b);
            }
            (Some(circuit_a), None) => {
                circuits[circuit_a].junction_boxes.push(*value_b);
                circuit_map.insert(*value_b, circuit_a);
            }
            (Some(circuit_a), Some(circuit_b)) => {
                if circuit_a == circuit_b {
                    continue;
                }

                let boxes_to_move = circuits[circuit_b].junction_boxes.clone();

                for moved_box in &boxes_to_move {
                    circuit_map.insert(*moved_box, circuit_a);
                }

                circuits[circuit_a].junction_boxes.extend(boxes_to_move);
                circuits[circuit_b].junction_boxes.clear();
            }
        }

        debug_circuits(&circuits);
    }

    circuits.sort_by_key(|circuit| Reverse(circuit.junction_boxes.len()));
    debug_circuits(&circuits);

    let answer: usize = circuits
        .iter()
        .map(|circuit| circuit.junction_boxes.len())
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("Answer: {}", answer);
}

fn debug_circuits(circuits: &Vec<Circuit>) {
    for circuit in circuits {
        debug!("{}", circuit);
    }
    debug!("{}", "");
}

fn find_closest_junction_boxes(junction_boxes: &[JunctionBox]) -> Vec<(usize, usize, usize)> {
    let mut distances: Vec<(usize, usize, usize)> = vec![];
    let mut seen: HashMap<usize, usize> = HashMap::new();

    for (i, a) in junction_boxes.iter().enumerate() {
        for (j, b) in junction_boxes.iter().enumerate() {
            if i < j {
                distances.push((i, j, math::distance_3d(b.into(), a.into()) as usize));
                seen.insert(i, j);
            }
        }
    }

    distances.sort_by_key(|(_, _, distance)| *distance);
    distances
}
