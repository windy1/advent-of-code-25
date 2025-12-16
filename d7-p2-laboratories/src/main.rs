#[cfg(not(debug_assertions))]
use std::collections::HashMap;

use aoc25::{char_grid::CharGrid, io};
#[cfg(debug_assertions)]
use indexmap::IndexMap;

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../input_example.txt");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.txt");
const START: char = 'S';
const BEAM: char = '|';
const SPLITTER: char = '^';
const EXPLORED: char = 'o';
const SPLITTER_ACTIVE: char = 'v';

#[derive(Debug, Clone, PartialEq, Eq)]
struct TachyonManifold {
    grid: CharGrid,
    start_x: usize,
    start_y: usize,
    num_timelines: usize,
    total_timelines: usize,
    num_splitters: usize,
    num_visited: usize,
    splitter_x: usize,
    splitter_y: usize,
    #[cfg(not(debug_assertions))]
    visited: HashMap<(usize, usize), usize>,
    #[cfg(debug_assertions)]
    visited: IndexMap<(usize, usize), usize>,
}

impl TachyonManifold {
    pub fn new(mut grid: CharGrid) -> Self {
        let (start_x, start_y) = grid.position_of(START).expect("start not found");
        let num_splitters = grid.iter().filter(|c| **c == SPLITTER).count();

        if cfg!(debug_assertions) {
            grid = grid.toggle_axes();
        } else {
            *grid.cell_width_mut() = 2;
        }

        TachyonManifold {
            grid,
            start_x,
            start_y,
            num_timelines: 0,
            total_timelines: 0,
            num_splitters,
            num_visited: 0,
            splitter_x: usize::MAX,
            splitter_y: usize::MAX,
            #[cfg(not(debug_assertions))]
            visited: HashMap::default(),
            #[cfg(debug_assertions)]
            visited: IndexMap::default(),
        }
    }

    fn run(&mut self) {
        self.split_beam(self.start_x, self.start_y, self.grid.clone())
    }

    fn split_beam(&mut self, x: usize, mut y: usize, mut timeline: CharGrid) {
        self.print(&timeline);

        let start_y = self.start_y;
        let start_x = self.start_x;

        if y > timeline.height() - 1 {
            // Reached the end of the manifold
            self.check_explored(&mut timeline);
            return;
        }

        if y == start_y && x == start_x {
            // Cast a beam to the first splitter
            y += 1;
            Self::cast_beam(&mut timeline, x, &mut y);
            self.split_beam(x, y, timeline.clone());
            self.total_timelines = *self.visited.get(&(x, y)).expect("node should exist");
            self.print(&timeline);
            return;
        }

        self.flip_splitter(&mut timeline, x, y);

        if self.visited.contains_key(&(x, y)) {
            return;
        }

        let lx: i32 = x as i32 - 1;
        let rx: i32 = x as i32 + 1;

        if timeline.contains(lx, y as i32) && timeline.get(lx as usize, y) != SPLITTER {
            // Split left
            let mut ly = y;
            Self::cast_beam(&mut timeline, lx as usize, &mut ly);
            self.split_beam(lx as usize, ly, timeline.clone());
            self.splitter_x = x;
            self.splitter_y = y;
        }

        if timeline.contains(rx, y as i32) && timeline.get(rx as usize, y) != SPLITTER {
            // Split right
            let mut ry = y;
            Self::cast_beam(&mut timeline, rx as usize, &mut ry);
            self.split_beam(rx as usize, ry, timeline.clone());
        }

        if self.splitter_x == x && self.splitter_y == y {
            return;
        }

        self.flip_splitter(&mut timeline, x, y);
        self.check_explored(&mut timeline);
    }

    fn flip_splitter(&mut self, timeline: &mut CharGrid, x: usize, y: usize) {
        if self.splitter_x != usize::MAX && self.splitter_y != usize::MAX {
            timeline.set(self.splitter_x, self.splitter_y, SPLITTER);
        }

        self.splitter_x = x;
        self.splitter_y = y;
        timeline.set(x, y, SPLITTER_ACTIVE);
    }

    fn cast_beam(timeline: &mut CharGrid, x: usize, y: &mut usize) {
        while timeline.contains(x as i32, *y as i32) && timeline.get(x, *y) != SPLITTER {
            timeline.set(x, *y, BEAM);
            *y += 1;
        }
    }

    fn check_explored(&mut self, timeline: &mut CharGrid) {
        let sx = self.splitter_x;
        let sy = self.splitter_y;

        if sx == usize::MAX || sy == usize::MAX {
            return;
        }

        let lx: i32 = sx as i32 - 1;
        let rx: i32 = sx as i32 + 1;

        let beam_explored = timeline.contains(lx, sy as i32)
            && timeline.get(lx as usize, sy) == BEAM
            && timeline.contains(rx, sy as i32)
            && timeline.get(rx as usize, sy) == BEAM;

        if !beam_explored {
            return;
        }

        // Both sides of the splitter have been explored
        let mut tmp_y = sy;
        Self::cast_beam(timeline, lx as usize, &mut tmp_y);

        if tmp_y < timeline.height() {
            // There exists a splitter towards the bottom-left
            self.num_timelines += self
                .visited
                .get(&(lx as usize, tmp_y))
                .expect("node should exist");
        } else {
            self.num_timelines += 1;
        }

        tmp_y = sy;
        Self::cast_beam(timeline, rx as usize, &mut tmp_y);

        if tmp_y < timeline.height() {
            // There exists a splitter towards the bottom-right
            self.num_timelines += self
                .visited
                .get(&(rx as usize, tmp_y))
                .expect("node should exist");
        } else {
            self.num_timelines += 1;
        }

        self.grid.set(sx, sy, EXPLORED);
        self.visited.insert((sx, sy), self.num_timelines);
        self.num_visited += 1;
        self.num_timelines = 0;
    }

    fn print(&self, timeline: &CharGrid) {
        io::clear_screen();

        if cfg!(debug_assertions) {
            println!("{}", self.grid);
        }

        println!("{}", timeline);
        println!("Visited: {}/{}", self.num_visited + 1, self.num_splitters);

        if cfg!(debug_assertions) {
            println!("{:?}", self.visited);
        }
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
    manifold.run();
    println!("Timelines: {}", manifold.total_timelines);
}
