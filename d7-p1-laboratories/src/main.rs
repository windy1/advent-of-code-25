use std::{thread, time::Duration};

use aoc25::char_grid::CharGrid;

const INPUT: &str = include_str!("../input.txt");
const START: char = 'S';
const BEAM: char = '|';
const SPLITTER: char = '^';

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct TachyonManifold {
    grid: CharGrid,
    start_x: usize,
    start_y: usize,
    current_y: usize,
    num_splits: usize,
}

impl TachyonManifold {
    pub fn new(grid: CharGrid) -> Self {
        let (start_x, start_y) = grid.position_of(START).expect("start not found");
        TachyonManifold {
            grid,
            start_x,
            start_y,
            current_y: start_y,
            num_splits: 0,
        }
    }

    pub fn next(&mut self) -> bool {
        let start_y = self.start_y;
        let start_x = self.start_x;
        let y = self.current_y;
        let grid = &mut self.grid;

        if y > grid.height() - 1 {
            return false;
        }

        if y == start_y {
            grid.set(start_x, start_y + 1, BEAM);
            self.current_y += 1;
            return true;
        }

        let row = grid.get_row(y);
        let mut new_beams: Vec<(usize, usize)> = vec![];

        for (x, ch) in row.iter().enumerate() {
            if *ch != SPLITTER {
                continue;
            }

            let above = grid.get(x, y - 1);

            if above != BEAM {
                continue;
            }

            // Beam is hitting splitter
            let mut did_split = false;
            new_beams.append(&mut Self::cast_beam(grid, x - 1, y, &mut did_split));
            new_beams.append(&mut Self::cast_beam(grid, x + 1, y, &mut did_split));

            if did_split {
                self.num_splits += 1;
            }
        }

        for (x, y) in new_beams {
            grid.set(x, y, BEAM);
        }

        self.current_y += 1;
        true
    }

    fn cast_beam(
        grid: &CharGrid,
        x: usize,
        mut y: usize,
        did_split: &mut bool,
    ) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = vec![];

        while grid.contains(x as i32, y as i32) && grid.get(x, y) != SPLITTER {
            *did_split = true;
            positions.push((x, y));
            y += 1;
        }

        positions
    }

    pub fn print(&self) {
        clear_screen();
        println!("{}", self.grid);
    }
}

impl From<&str> for TachyonManifold {
    fn from(value: &str) -> Self {
        TachyonManifold::new(value.into())
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let mut manifold: TachyonManifold = INPUT.into();

    while manifold.next() {
        manifold.print();
        thread::sleep(Duration::from_millis(50));
    }

    println!("Answer: {}", manifold.num_splits);
}

fn clear_screen() {
    print!("\x1B[3J\x1B[H\x1B[2J");
}
