use aoc25::char_grid::CharGrid;

const INPUT: &str = include_str!("../input.txt");
const MAX_ADJACENT_ROLLS: u32 = 3;
const ROLL: char = '@';

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let mut input: CharGrid = INPUT.into();
    let mut output = input.clone();
    let mut answer = 0;

    loop {
        let num_rolls = calc_num_accessible_rolls(&input, &mut output);
        input = output;
        output = input.clone();
        answer += num_rolls;
        if num_rolls == 0 {
            break;
        }
    }

    println!("{}", output);
    println!("Answer: {}", answer);
}

fn calc_num_accessible_rolls(input: &CharGrid, output: &mut CharGrid) -> usize {
    let mut result = 0;

    for (x, y) in input.coordinates_iter() {
        if input.get(x, y) != ROLL {
            continue;
        }

        let mut count = 0;

        for (ax, ay) in input.neighbors_iter(x, y).chain(input.diagonals_iter(x, y)) {
            if input.get(ax, ay) == ROLL {
                count += 1;
                if count > MAX_ADJACENT_ROLLS {
                    break;
                }
            }
        }

        if count <= MAX_ADJACENT_ROLLS {
            output.set(x, y, 'x');
            result += 1;
        }
    }

    result
}
