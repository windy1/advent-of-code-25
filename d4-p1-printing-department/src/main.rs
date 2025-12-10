use aoc25::char_grid::CharGrid;

const INPUT: &str = include_str!("../input.txt");
const MAX_ADJACENT_ROLLS: u32 = 3;
const ROLL: char = '@';

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let grid: CharGrid = INPUT.into();
    let mut result_grid = grid.clone();
    let mut answer = 0;

    for (x, y) in grid.coordinates_iter() {
        if grid.get(x, y) != ROLL {
            continue;
        }

        let mut count = 0;

        for (ax, ay) in grid.neighbors_iter(x, y).chain(grid.diagonals_iter(x, y)) {
            if grid.get(ax, ay) == ROLL {
                count += 1;
                if count > MAX_ADJACENT_ROLLS {
                    break;
                }
            }
        }

        if count <= MAX_ADJACENT_ROLLS {
            result_grid.set(x, y, 'x');
            answer += 1;
        }
    }

    println!("{}", result_grid);
    println!("Answer: {}", answer);
}
