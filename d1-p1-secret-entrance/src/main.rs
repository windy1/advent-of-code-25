const INPUT: &str = include_str!("../input_example.txt");
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

        println!("Moving from {} to {}", position, new_position);
        position = new_position;

        if position == 0 {
            password += 1;
        }
    }

    println!("Password: {}", password);
}
