#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

use std::{
    collections::HashSet,
    iter,
    ops::Coroutine,
    thread,
    time::{Duration, Instant},
};

use aoc25::{char_grid::CharGrid, io, range::bidirectional_range, util::string::format_duration};

#[cfg(debug_assertions)]
const INPUT: &str = include_str!("../input_example.txt");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.txt");
const TILE_RED: char = '#';
const TILE_OTHER: char = '.';
const TILE_PAINTED: char = 'o';

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct MovieTheater {
    #[cfg(debug_assertions)]
    initial: CharGrid,
    grid: CharGrid,
    red_tile_positions: Vec<(usize, usize)>,
}

type Rect = (usize, usize, usize, usize);

impl MovieTheater {
    pub fn new(red_tile_positions: Vec<(usize, usize)>) -> Self {
        let width = red_tile_positions
            .iter()
            .map(|(x, _)| x)
            .max()
            .expect("could not determine width")
            + 2;

        let height = red_tile_positions
            .iter()
            .map(|(_, y)| y)
            .max()
            .expect("could not determine height")
            + 2;

        let mut grid = CharGrid::with_default_char(width, height, TILE_OTHER).toggle_axes();

        for (x, y) in &red_tile_positions {
            grid.set(*x, *y, TILE_RED);
        }

        MovieTheater {
            #[cfg(debug_assertions)]
            initial: grid.clone(),
            grid,
            red_tile_positions,
        }
    }

    pub fn opposite_corners_gen(
        &self,
        x: usize,
        y: usize,
    ) -> impl Coroutine<Yield = (usize, usize), Return = ()> + use<'_> {
        #[coroutine]
        move || {
            for (other_x, other_y) in &self.red_tile_positions {
                if x == *other_x && y == *other_y {
                    continue;
                }

                if *other_x != x || *other_y != y {
                    yield (*other_x, *other_y)
                }
            }
        }
    }

    pub fn draw_rect(&mut self, rect: Rect) {
        let (x1, y1, x2, y2) = (rect.0 as i64, rect.1 as i64, rect.2 as i64, rect.3 as i64);
        let dx = x2 - x1;
        let dy = y2 - y1;

        for x in bidirectional_range(x1, dx) {
            for y in bidirectional_range(y1, dy) {
                self.grid.set(x as usize, y as usize, TILE_PAINTED);
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn reset(&mut self) {
        self.grid = self.initial.clone();
    }

    #[cfg(not(debug_assertions))]
    pub fn reset(&mut self) {}
}

impl From<&str> for MovieTheater {
    fn from(value: &str) -> Self {
        let red_tile_positions: Vec<(usize, usize)> = value
            .lines()
            .map(|line| line.split(',').collect::<Vec<&str>>())
            .map(parse_coords)
            .collect();
        MovieTheater::new(red_tile_positions)
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    println!("Loading...");

    let mut theater: MovieTheater = INPUT.into();
    let positions = theater.red_tile_positions.to_vec();

    println!(
        "Loaded {}x{} grid with {} red tiles",
        theater.grid.width(),
        theater.grid.height(),
        positions.len()
    );

    let mut max_area = 0;
    let start = Instant::now();
    let num_pairs = positions.len() * (positions.len() - 1) / 2;
    let mut pairs_tested: HashSet<Rect> = HashSet::default();

    for (tile_x, tile_y) in positions {
        let corners: Vec<_> =
            iter::from_coroutine(theater.opposite_corners_gen(tile_x, tile_y)).collect();

        for (corner_x, corner_y) in corners {
            if pairs_tested.contains(&(corner_x, corner_y, tile_x, tile_y)) {
                continue;
            }

            let width = ((tile_x as i64 - corner_x as i64).abs() + 1) as usize;
            let height = ((tile_y as i64 - corner_y as i64).abs() + 1) as usize;
            let area = width * height;
            let rect = (tile_x, tile_y, corner_x, corner_y);

            if area > max_area {
                max_area = area;
            }

            pairs_tested.insert(rect);

            if cfg!(debug_assertions) {
                theater.reset();
                theater.draw_rect(rect);
            }

            render(Context {
                theater: &theater,
                tile_x,
                tile_y,
                corner_x,
                corner_y,
                width,
                height,
                area,
                max_area,
                start,
                pairs_tested: pairs_tested.len(),
                num_pairs,
            })
        }
    }
}

struct Context<'a> {
    theater: &'a MovieTheater,
    tile_x: usize,
    tile_y: usize,
    corner_x: usize,
    corner_y: usize,
    width: usize,
    height: usize,
    area: usize,
    max_area: usize,
    start: Instant,
    pairs_tested: usize,
    num_pairs: usize,
}

fn render(context: Context) {
    if cfg!(debug_assertions) {
        thread::sleep(Duration::from_millis(200));
    }

    io::clear_screen();

    if cfg!(debug_assertions) {
        println!("{}", context.theater.grid);
    }

    let pairs_tested = context.pairs_tested;
    let num_pairs = context.num_pairs;
    let progress = (pairs_tested as f64 / num_pairs as f64) * 100.0;

    println!("Tile: ({}, {})", context.tile_x, context.tile_y);
    println!("Corner: ({}, {})", context.corner_x, context.corner_y);
    println!("{} x {} = {}", context.width, context.height, context.area);
    println!("Max: {}", context.max_area);
    println!("Tested: {:.2} ({}/{})", progress, pairs_tested, num_pairs);
    println!("Runtime: {}", format_duration(context.start.elapsed()));
}

fn parse_coords(coords: Vec<&str>) -> (usize, usize) {
    let x = coords[0].parse::<usize>().expect("invalid x");
    let y = coords[1].parse::<usize>().expect("invalid y");
    (x, y)
}
