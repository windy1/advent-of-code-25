#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

use std::{
    fmt::{self, Display, Formatter},
    fs, iter,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use aoc25::{
    char_grid::CharGrid,
    io::{self, ReadProgress},
    range::bidirectional_range,
    util::string::{format_duration, format_mem_size},
};
use clap::Parser;
use d9_p2_movie_theater::args::Args;
use indexmap::IndexMap;

#[cfg(debug_assertions)]
// const INPUT: &str = include_str!("../input_example.txt");
// const INPUT: &str = include_str!("../input_example_2.txt");
// const INPUT: &str = include_str!("../input_example_3.txt");
const INPUT: &str = include_str!("../input_example_4.txt");
#[cfg(not(debug_assertions))]
const INPUT: &str = include_str!("../input.txt");
const TILE_RED: char = '#';
const TILE_GREEN: char = 'X';
const TILE_OTHER: char = '.';
const TILE_PAINTED: char = 'o';
const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
const DELAY: Duration = Duration::from_millis(0);

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct MovieTheater {
    #[cfg(debug_assertions)]
    initial: CharGrid,
    grid: CharGrid,
    red_tile_positions: Vec<(usize, usize)>,
}

impl MovieTheater {
    pub fn new(mut red_tile_positions: Vec<(usize, usize)>) -> Self {
        let width = Self::calc_width(&red_tile_positions);
        let height = Self::calc_height(&red_tile_positions);
        let mut grid = CharGrid::with_default_char(width, height, TILE_OTHER).toggle_axes();

        red_tile_positions.sort();

        Self::draw_red_tiles(&mut grid, &red_tile_positions);
        Self::draw_green_tile_border(&mut grid, &red_tile_positions);
        Self::fill_green_tiles(&mut grid);

        debug_grid(&grid);

        MovieTheater {
            #[cfg(debug_assertions)]
            initial: grid.clone(),
            grid,
            red_tile_positions,
        }
    }

    fn calc_width(red_tile_positions: &[(usize, usize)]) -> usize {
        red_tile_positions
            .iter()
            .map(|(x, _)| x)
            .max()
            .expect("could not determine width")
            + 2
    }

    fn calc_height(red_tile_positions: &[(usize, usize)]) -> usize {
        red_tile_positions
            .iter()
            .map(|(_, y)| y)
            .max()
            .expect("could not determine height")
            + 2
    }

    fn draw_red_tiles(grid: &mut CharGrid, positions: &[(usize, usize)]) {
        println!("Drawing red tiles...");
        for (x, y) in positions {
            grid.set(*x, *y, TILE_RED);
        }
    }

    fn draw_green_tile_border(grid: &mut CharGrid, red_tile_positions: &[(usize, usize)]) {
        println!("Drawing green border tiles...");

        let mut visited: IndexMap<(usize, usize), usize> = IndexMap::default();
        let first = red_tile_positions[0] as (usize, usize);
        let (mut corner_x, mut corner_y) = first as (usize, usize);
        let (first_x, first_y) = first;
        let mut direction_index = 0;
        let mut found_connection = false;

        while visited.len() < red_tile_positions.len() {
            while direction_index < DIRECTIONS.len() {
                let (dx, dy) = DIRECTIONS[direction_index];
                let start_x = corner_x as i64 + dx;
                let start_y = corner_y as i64 + dy;

                if !grid.contains(start_x, start_y) {
                    // Cannot go this direction
                    direction_index += 1;
                    continue;
                }

                // Try and find a connected corner
                let cast: Vec<_> = grid
                    .raycast_iter(
                        start_x as usize,
                        start_y as usize,
                        dx,
                        dy,
                        &[TILE_RED, TILE_GREEN],
                    )
                    .collect();

                let (last_x, last_y) = match cast.last() {
                    Some(last) => *last,
                    None => {
                        // There is a red or green tile directly adjacent
                        direction_index += 1;
                        continue;
                    }
                };

                let (hit_x, hit_y) = (last_x as i64 + dx, last_y as i64 + dy);

                if !grid.contains(hit_x, hit_y) {
                    // We did not find a red or green tile
                    direction_index += 1;
                    continue;
                }

                let hit_ch = grid.get(hit_x as usize, hit_y as usize);

                if hit_ch == TILE_GREEN {
                    // Intersects another border
                    direction_index += 1;
                    continue;
                }

                if (hit_x != first_x as i64 || hit_y != first_y as i64)
                    && visited.contains_key(&(hit_x as usize, hit_y as usize))
                {
                    // The hit corner is already connected to another node, and it is not the first
                    // corner which would complete the border.
                    direction_index += 1;
                    continue;
                }

                visited.insert((corner_x, corner_y), direction_index + 1);

                for (border_x, border_y) in cast {
                    grid.set(border_x, border_y, TILE_GREEN);
                }

                found_connection = true;
                direction_index = 0;
                (corner_x, corner_y) = (hit_x as usize, hit_y as usize);
            }

            if !found_connection {
                // There is no connection from the current corner
                // Backtrack and try the next direction at the previous node
                // If the visited list is empty, we could not find a connection from the first node
                let mut pop_visited = || {
                    let last = visited.pop().expect("nowhere to go");
                    let ((last_x, last_y), last_direction) = last;
                    let (dx, dy) = DIRECTIONS[last_direction - 1];

                    // Undo border
                    let cast: Vec<_> = grid
                        .raycast_iter(
                            (last_x as i64 + dx) as usize,
                            (last_y as i64 + dy) as usize,
                            dx,
                            dy,
                            &[TILE_RED],
                        )
                        .collect();

                    for (border_x, border_y) in cast {
                        grid.set(border_x, border_y, TILE_OTHER);
                    }

                    last
                };

                let mut last = pop_visited();

                while last.1 == DIRECTIONS.len() {
                    // We have exhausted all directions for the last node
                    // Backtrack until there is still directions to test
                    last = pop_visited();
                }

                // Resume from the backtracked node
                let (last_corner, last_direction) = last;
                direction_index = last_direction;
                (corner_x, corner_y) = last_corner;
            }

            found_connection = false;
        }
    }

    fn fill_green_tiles(grid: &mut CharGrid) {
        io::hide_cursor();

        let grid_width = grid.width();
        let grid_height = grid.height();
        let (mut start_x, mut start_y) = (0, 0);

        debug_grid(grid);

        for (x, y) in grid.coordinates_iter() {
            let current = grid.get(x, y);

            if current != TILE_GREEN {
                continue;
            }

            let next_x = x + 1;

            if grid.contains(next_x as i64, y as i64) && grid.get(next_x, y) == TILE_OTHER {
                // Green tile followed by empty, found starting position
                (start_x, start_y) = (next_x, y);
                break;
            }
        }

        let (mut x, mut y) = (start_x, start_y);

        loop {
            let row = y + 1;
            let progress = row as f64 / grid_height as f64 * 100.0;

            debug_grid(grid);

            print!(
                "\rDrawing green fill tiles... {:.2}% ({}/{})",
                progress, row, grid_height,
            );

            if y > grid_height {
                break;
            }

            if x > grid_width - 1 {
                x = 0;
                y += 1;
                continue;
            }

            // Fill right until we hit a border
            let mut current = grid.get(x, y);

            if current == TILE_OTHER {
                grid.set(x, y, TILE_GREEN);
                x += 1;
                continue;
            }

            let rewind = |current: &mut char, x: &mut usize, y: &mut usize| {
                let mut scan_x = *x - 1;
                let mut along_edge = false;
                let mut outside = false;

                loop {
                    if scan_x < 1 {
                        // Reached end of grid
                        break;
                    }

                    let scan = grid.get(scan_x, *y);
                    let left = grid.get(scan_x - 1, *y);
                    let right = grid.get(scan_x + 1, *y);

                    if !outside && !along_edge && scan != TILE_OTHER && left == TILE_OTHER {
                        // Reached a border from inside
                        // Move position to one after border
                        *x = scan_x + 1;
                        scan_x -= 1;
                        outside = true;
                        continue;
                    }

                    if outside && scan == TILE_OTHER {
                        // Space outside
                        scan_x -= 1;
                        continue;
                    }

                    if outside && right == TILE_OTHER && scan == TILE_GREEN {
                        // Reached a border from outside
                        // Move position to one before border
                        *x = scan_x - 1;
                        scan_x -= 1;
                        outside = false;
                        continue;
                    }

                    if !along_edge && scan == TILE_RED {
                        // Encountered a corner from inside
                        along_edge = true;
                        outside = false;
                        scan_x -= 1;
                        continue;
                    }

                    if along_edge {
                        if scan == TILE_RED {
                            scan_x -= 1;
                            let mut inside = true;

                            // Reached the end of an edge
                            for (dx, dy) in [(0, -1), (-1, 0), (0, 1)] {
                                // Check up, left, and down for a border
                                let cast_x = scan_x as i64 + dx;
                                let cast_y = *y as i64 + dy;

                                if !grid.contains(cast_x, cast_y) {
                                    inside = false;
                                    break;
                                }

                                if grid.get(cast_x as usize, cast_y as usize) != TILE_OTHER {
                                    continue;
                                }

                                let cast = grid.raycast_iter(
                                    cast_x as usize,
                                    cast_y as usize,
                                    dx,
                                    dy,
                                    &[TILE_RED, TILE_GREEN],
                                );

                                let (last_x, last_y) = match cast.last() {
                                    Some(last) => last,
                                    None => {
                                        // Border not found for current direction
                                        inside = false;
                                        break;
                                    }
                                };

                                let (hit_x, hit_y) = (last_x as i64 + dx, last_y as i64 + dy);

                                if !grid.contains(hit_x, hit_y) {
                                    // Reached end of grid
                                    inside = false;
                                    break;
                                }
                            }

                            along_edge = false;

                            if inside {
                                *x = scan_x;
                                continue;
                            }

                            outside = true;
                        }

                        scan_x -= 1;
                        continue;
                    }

                    scan_x -= 1;
                    *x = scan_x;
                }

                *current = grid.get(*x, *y);
            };

            let advance = |x: &mut usize, y: &mut usize| {
                let mut scan_x = *x;
                let mut area_found = false;
                let mut along_edge = false;
                let mut outside = false;

                // Try to jump to a next area
                loop {
                    if scan_x > grid_width - 2 {
                        // No room for another area
                        break;
                    }

                    let scan = grid.get(scan_x, *y);
                    let right = grid.get(scan_x + 1, *y);
                    let left = grid.get(scan_x - 1, *y);

                    if !outside && !along_edge && scan != TILE_OTHER && right == TILE_OTHER {
                        // Reached a border from inside
                        scan_x += 1;
                        outside = true;
                        continue;
                    }

                    if outside && scan == TILE_OTHER {
                        // Space outside
                        scan_x += 1;
                        continue;
                    }

                    if outside && scan == TILE_GREEN && left == TILE_OTHER {
                        // Reached a border from outside
                        scan_x += 1;
                        *x = scan_x;
                        area_found = true;
                        break;
                    }

                    if !along_edge && scan == TILE_RED {
                        // Encountered a corner from inside
                        along_edge = true;
                        outside = false;
                        scan_x += 1;
                        continue;
                    }

                    if along_edge {
                        if scan == TILE_RED {
                            scan_x += 1;
                            let mut inside = true;

                            // Reached the end of an edge
                            // Check up, right, and down for a border
                            for (dx, dy) in [(0, -1), (1, 0), (0, 1)] {
                                let cast_x = scan_x as i64 + dx;
                                let cast_y = *y as i64 + dy;

                                if !grid.contains(cast_x, cast_y) {
                                    inside = false;
                                    break;
                                }

                                if grid.get(cast_x as usize, cast_y as usize) != TILE_OTHER {
                                    continue;
                                }

                                let cast = grid.raycast_iter(
                                    cast_x as usize,
                                    cast_y as usize,
                                    dx,
                                    dy,
                                    &[TILE_RED, TILE_GREEN],
                                );

                                let (last_x, last_y) = match cast.last() {
                                    Some(last) => last,
                                    None => {
                                        // Border not found for current direction
                                        inside = false;
                                        break;
                                    }
                                };

                                let (hit_x, hit_y) = (last_x as i64 + dx, last_y as i64 + dy);

                                if !grid.contains(hit_x, hit_y) {
                                    // Reached end of grid
                                    inside = false;
                                    break;
                                }
                            }

                            along_edge = false;

                            if inside {
                                *x = scan_x;
                                area_found = true;
                                break;
                            }

                            outside = true;
                        }
                        scan_x += 1;
                        continue;
                    }

                    scan_x += 1;
                    *x = scan_x;
                }

                area_found
            };

            if advance(&mut x, &mut y) {
                continue;
            }

            // Rewind left
            rewind(&mut current, &mut x, &mut y);

            if y == grid_height - 2 {
                break;
            }

            let mut finished = false;

            // Find path downward
            loop {
                let below = grid.get(x, y + 1);
                let current = grid.get(x, y);

                if current == TILE_OTHER {
                    if advance(&mut x, &mut y) {
                        continue;
                    }
                    break;
                }

                if below == TILE_OTHER {
                    break;
                }

                x += 1;

                if x == grid_width - 2 {
                    finished = true;
                    break;
                }
            }

            if finished || current == TILE_OTHER {
                break;
            }

            y += 1;

            let left = grid.get(x - 1, y);

            if left != TILE_OTHER {
                continue;
            }

            rewind(&mut current, &mut x, &mut y);

            // let mut below = grid.get(x, y + 1);

            // while below != TILE_OTHER {
            //     x += 1;
            //     current = grid.get(x, y);
            //     below = grid.get(x, y + 1);
            // }

            // if current == TILE_OTHER {
            //     break;
            // }

            // y += 1;

            // let left = grid.get(x - 1, y);

            // if left != TILE_OTHER {
            //     continue;
            // }

            // // We need to rewind left on the new row too
            // rewind_left(&mut current, &mut x, &mut y);
        }

        io::show_cursor();
        println!();
    }

    pub fn opposite_corners_iter(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        iter::from_coroutine(
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
            },
        )
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

    pub fn is_rect_valid(&self, rect: Rect) -> bool {
        let (x1, y1, x2, y2) = (rect.0 as i64, rect.1 as i64, rect.2 as i64, rect.3 as i64);
        let dx = x2 - x1;
        let dy = y2 - y1;

        for x in bidirectional_range(x1, dx) {
            for y in bidirectional_range(y1, dy) {
                if self.initial().get(x as usize, y as usize) == TILE_OTHER {
                    return false;
                }
            }
        }

        true
    }

    #[cfg(debug_assertions)]
    pub fn initial(&self) -> &CharGrid {
        &self.initial
    }

    #[cfg(not(debug_assertions))]
    pub fn initial(&self) -> &CharGrid {
        &self.grid
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

impl From<CharGrid> for MovieTheater {
    fn from(grid: CharGrid) -> Self {
        let red_tile_positions: Vec<_> = grid
            .coordinates_iter()
            .filter(|(x, y)| grid.get(*x, *y) == TILE_RED)
            .collect();

        MovieTheater {
            #[cfg(debug_assertions)]
            initial: grid.clone(),
            grid,
            red_tile_positions,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Rect(usize, usize, usize, usize);

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(({}, {}), ({}, {}))", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    // too high: 4594510710
    // too high: 4594510709
    // too low: 93009

    println!("Loading...");

    let args = Args::parse();
    let mut theater = load_theater(&args.bake, &args.input);
    let positions = theater.red_tile_positions.to_vec();

    println!(
        "Loaded {}x{} grid with {} red tiles",
        theater.grid.width(),
        theater.grid.height(),
        positions.len()
    );

    let mut max_area = 0;
    let mut max_position = Rect::default();
    let start = Instant::now();
    let num_pairs = positions.len() * (positions.len() - 1) / 2;
    let mut pairs_tested = 0;

    debug_grid(&theater.grid);

    for (i, &(tile_x, tile_y)) in positions.iter().enumerate() {
        let corners: Vec<_> = theater
            .opposite_corners_iter(tile_x, tile_y)
            .enumerate()
            .collect();

        for (j, (corner_x, corner_y)) in corners {
            if i <= j {
                continue;
            }

            let width = ((tile_x as i64 - corner_x as i64).abs() + 1) as usize;
            let height = ((tile_y as i64 - corner_y as i64).abs() + 1) as usize;
            let area = width * height;
            let rect = Rect(tile_x, tile_y, corner_x, corner_y);
            let valid = theater.is_rect_valid(rect);

            if area > max_area && valid {
                max_area = area;
                max_position = rect;
            }

            if cfg!(debug_assertions) {
                theater.reset();
                theater.draw_rect(rect);
            }

            pairs_tested += 1;

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
                max_position,
                start,
                pairs_tested,
                num_pairs,
                valid,
            })
        }
    }
}

fn load_theater(bake: &Option<PathBuf>, input: &Option<PathBuf>) -> MovieTheater {
    if let Some(output) = bake {
        println!("Parsing input...");
        let mt: MovieTheater = INPUT.into();
        println!("Writing baked input...");
        fs::write(output, mt.grid.to_raw()).expect("could not write output");
        return mt;
    }

    if let Some(input) = input {
        io::hide_cursor();
        let baked_input: String = io::read_to_string_with_progress(input, read_input_progress)
            .expect("could not read baked input");
        io::show_cursor();
        println!();
        println!("Parsing grid...");
        let grid: CharGrid = baked_input.as_str().into();
        return grid.into();
    }

    INPUT.into()
}

fn read_input_progress(progress: ReadProgress) {
    let bytes_read = progress.bytes_read;
    let total_bytes = progress.total_bytes;
    let progress = (bytes_read as f64 / total_bytes as f64) * 100.0;
    print!(
        "\rLoading baked input... {:.2}% ({}/{})",
        progress, bytes_read, total_bytes
    );
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
    max_position: Rect,
    start: Instant,
    pairs_tested: usize,
    num_pairs: usize,
    valid: bool,
}

fn render(context: Context) {
    if cfg!(debug_assertions) {
        thread::sleep(DELAY);
    }

    io::clear_screen();

    debug_grid(context.theater.initial());
    debug_grid(&context.theater.grid);

    let pairs_tested = context.pairs_tested;
    let num_pairs = context.num_pairs;
    let progress = (pairs_tested as f64 / num_pairs as f64) * 100.0;
    let mem_size = format_mem_size(context.theater.grid.mem_size());

    println!("Tile: ({}, {})", context.tile_x, context.tile_y);
    println!("Corner: ({}, {})", context.corner_x, context.corner_y);
    println!("{} x {} = {}", context.width, context.height, context.area);
    println!("Valid: {}", context.valid);
    println!("Max: {}@{}", context.max_area, context.max_position);
    println!("Tested: {:.2}% ({}/{})", progress, pairs_tested, num_pairs);
    println!("Grid Memory: {}", mem_size);
    println!("Runtime: {}", format_duration(context.start.elapsed()));
}

fn parse_coords(coords: Vec<&str>) -> (usize, usize) {
    let x = coords[0].parse::<usize>().expect("invalid x");
    let y = coords[1].parse::<usize>().expect("invalid y");
    (x, y)
}

#[cfg(debug_assertions)]
fn debug_grid(grid: &CharGrid) {
    io::clear_screen();
    println!("{}", grid);
}

#[cfg(not(debug_assertions))]
fn debug_grid(_grid: &CharGrid) {}
