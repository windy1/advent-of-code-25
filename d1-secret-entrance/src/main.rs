const INPUT: &str = include_str!("../input_example.txt");

#[derive(Debug)]
struct Rotation {
    right: bool,
    distance: usize,
}

fn main() {
    let rotations = INPUT
        .lines()
        .map(|line| {
            let direction = &line[0..1];
            let distance = line[1..].parse::<usize>().unwrap();
            Rotation {
                right: direction == "R",
                distance,
            }
        })
        .collect::<Vec<Rotation>>();

    println!("{:?}", rotations);
}
