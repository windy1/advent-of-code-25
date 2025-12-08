const INPUT: &str = include_str!("../input.txt");
const MAX_POSITION: usize = 99;
const INITIAL_POSITION: usize = 50;

#[derive(Debug)]
struct Rotation {
    right: bool,
    distance: usize,
}

impl From<&str> for Rotation {
    fn from(line: &str) -> Self {
        let direction = &line[0..1];
        let distance = line[1..].parse::<usize>().unwrap();
        Rotation {
            right: direction == "R",
            distance,
        }
    }
}

fn main() {
    let rotations = INPUT.lines().map(Rotation::from).collect::<Vec<Rotation>>();
    let mut position: usize = INITIAL_POSITION;
    let mut password = 0;

    for rotation in &rotations {
        let new_position = if rotation.right {
            (position + rotation.distance) % (MAX_POSITION + 1)
        } else {
            (position + MAX_POSITION + 1 - rotation.distance % (MAX_POSITION + 1))
                % (MAX_POSITION + 1)
        };

        println!("Moving from {} to {}", position, new_position);
        position = new_position;

        if position == 0 {
            password += 1;
        }
    }

    println!("Password: {}", password);
}
