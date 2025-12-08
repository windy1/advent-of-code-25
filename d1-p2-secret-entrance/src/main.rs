use std::fmt::{self, Display, Formatter};

const INPUT: &str = include_str!("../input.txt");
const MAX_POSITION: i32 = 99;
const INITIAL_POSITION: i32 = 50;

#[derive(Debug)]
struct Rotation {
    right: bool,
    distance: i32,
}

impl From<&str> for Rotation {
    fn from(line: &str) -> Self {
        let direction = &line[0..1];
        let distance = line[1..].parse::<i32>().unwrap();
        Rotation {
            right: direction == "R",
            distance,
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.right { "R" } else { "L" }, self.distance)
    }
}

fn main() {
    let rotations = INPUT.lines().map(Rotation::from).collect::<Vec<Rotation>>();
    let mut position: i32 = INITIAL_POSITION;
    let mut password = 0;

    for rotation in &rotations {
        let delta = if rotation.right {
            rotation.distance
        } else {
            -rotation.distance
        };

        let new_position =
            ((position + delta) % (MAX_POSITION + 1) + (MAX_POSITION + 1)) % (MAX_POSITION + 1);

        let zero_passes = if rotation.right {
            let distance = rotation.distance;
            let zero_distance = (MAX_POSITION + 1) - position;

            if distance >= zero_distance {
                1 + (distance - zero_distance) / (MAX_POSITION + 1)
            } else {
                0
            }
        } else {
            let distance = rotation.distance;
            let zero_distance = position;

            if distance >= zero_distance && zero_distance > 0 {
                1 + (distance - zero_distance) / (MAX_POSITION + 1)
            } else if distance >= zero_distance && zero_distance == 0 {
                distance / (MAX_POSITION + 1)
            } else {
                0
            }
        };

        println!(
            "[{}] Moving from {} to {} (crossed zero {} times)",
            rotation, position, new_position, zero_passes
        );

        password += zero_passes;
        position = new_position;
    }

    println!("Password: {}", password);
}
