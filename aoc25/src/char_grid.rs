use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Flatten;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CharGrid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    axes_enabled: bool,
    cell_width: usize,
}

pub trait CoordIter = Iterator<Item = (usize, usize)>;

impl CharGrid {
    const DEFAULT_CHAR: char = '?';

    pub fn new(width: usize, height: usize) -> Self {
        CharGrid::with_default_char(width, height, Self::DEFAULT_CHAR)
    }

    pub fn with_default_char(width: usize, height: usize, default_char: char) -> Self {
        CharGrid {
            width,
            height,
            data: vec![vec![default_char; width]; height],
            axes_enabled: false,
            cell_width: 3,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    pub fn get_row(&self, y: usize) -> &[char] {
        &self.data[y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
    }

    pub fn position_of(&self, value: char) -> Option<(usize, usize)> {
        for (x, y) in self.coordinates_iter() {
            if self.get(x, y) == value {
                return Some((x, y));
            }
        }
        None
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if width == self.width && height == self.height {
            return;
        }

        if height != self.height {
            self.data.resize(height, vec![Self::DEFAULT_CHAR; width]);
        }

        if width != self.width {
            for row in &mut self.data {
                row.resize(width, Self::DEFAULT_CHAR);
            }
        }

        self.width = width;
        self.height = height;
    }

    pub fn toggle_axes(mut self) -> Self {
        self.axes_enabled = !self.axes_enabled;
        self
    }

    pub fn cell_width(&self) -> usize {
        self.cell_width
    }

    pub fn cell_width_mut(&mut self) -> &mut usize {
        &mut self.cell_width
    }

    pub fn push_x(&mut self, y: usize, value: char) {
        self.resize(self.width + 1, self.height);
        self.set(self.width - 1, y, value);
    }

    pub fn push_y(&mut self, x: usize, value: char) {
        self.resize(self.width, self.height + 1);
        self.set(x, self.height - 1, value);
    }

    pub fn coordinates_iter(&self) -> impl CoordIter {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
    }

    pub fn neighbors_iter(&self, x: usize, y: usize) -> impl CoordIter {
        self.deltas_iter(x, y, &[(0, -1), (1, 0), (0, 1), (-1, 0)])
    }

    pub fn diagonals_iter(&self, x: usize, y: usize) -> impl CoordIter {
        self.deltas_iter(x, y, &[(-1, -1), (1, -1), (1, 1), (-1, 1)])
    }

    fn deltas_iter(&self, x: usize, y: usize, deltas: &[(i32, i32)]) -> impl CoordIter {
        let x = x as i32;
        let y = y as i32;
        deltas
            .iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(|(nx, ny)| self.contains(*nx, *ny))
            .map(|(nx, ny)| (nx as usize, ny as usize))
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = &Vec<char>> {
        self.data.iter()
    }

    pub fn columns_iter(&self) -> impl Iterator<Item = Vec<char>> {
        Columns::new(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.data.iter().flatten()
    }
}

impl<'a> IntoIterator for &'a CharGrid {
    type Item = &'a char;
    type IntoIter = Flatten<Iter<'a, Vec<char>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().flatten()
    }
}

struct Columns<'a> {
    grid: &'a CharGrid,
    x: usize,
}

impl<'a> Columns<'a> {
    pub fn new(grid: &'a CharGrid) -> Self {
        Columns { grid, x: 0 }
    }
}

impl Iterator for Columns<'_> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let grid = &self.grid;
        let x = self.x;

        if x > grid.width() - 1 {
            return None;
        }

        let mut column: Vec<char> = vec![CharGrid::DEFAULT_CHAR; grid.height];

        for (y, value) in column.iter_mut().enumerate() {
            *value = grid.get(x, y);
        }

        self.x += 1;
        Some(column)
    }
}

impl Default for CharGrid {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Display for CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cell_width = self.cell_width;

        if self.axes_enabled {
            write!(f, "{:>3}", " ")?;
            for x in 0..self.width() {
                write!(f, "{:>cell_width$}", x.to_string())?;
            }
            writeln!(f)?;
        }

        for (y, row) in self.data.iter().enumerate() {
            if self.axes_enabled {
                write!(f, "{:>cell_width$}", y.to_string())?;
            }

            for c in row {
                write!(f, "{:>cell_width$}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for CharGrid {
    fn from(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut grid = CharGrid::new(width, height);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.data[y][x] = c;
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let grid = CharGrid::new(10, 20);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 20);
        assert_eq!(grid.size(), 200);
        assert_eq!(grid.get(0, 0), '?');
    }

    #[test]
    fn contains() {
        let grid = CharGrid::new(10, 20);
        assert!(grid.contains(0, 0));
        assert!(grid.contains(9, 0));
        assert!(grid.contains(9, 19));
        assert!(grid.contains(0, 19));
        assert!(grid.contains(5, 5));
        assert!(!grid.contains(-1, -1));
        assert!(!grid.contains(-1, 0));
        assert!(!grid.contains(0, -1));
        assert!(!grid.contains(10, 20));
        assert!(!grid.contains(10, 0));
        assert!(!grid.contains(20, 0));
    }

    #[test]
    fn set() {
        let mut grid = CharGrid::new(10, 20);
        grid.set(0, 0, 'A');
        assert_eq!(grid.get(0, 0), 'A');
    }

    #[test]
    fn coordinates_iter() {
        let grid = CharGrid::new(5, 5);
        let expected = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
        ];
        let actual: Vec<(usize, usize)> = grid.coordinates_iter().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn neighbors_iter() {
        let grid = CharGrid::new(6, 6);
        let expected = vec![(3, 2), (4, 3), (3, 4), (2, 3)];
        let actual: Vec<(usize, usize)> = grid.neighbors_iter(3, 3).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn diagonals_iter() {
        let grid = CharGrid::new(6, 6);
        let expected = vec![(2, 2), (4, 2), (4, 4), (2, 4)];
        let actual: Vec<(usize, usize)> = grid.diagonals_iter(3, 3).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_str() {
        let str = "AAA
ABA
AAA
AAA";
        println!("{}", str);
        let grid: CharGrid = str.into();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 4);
        assert_eq!(grid.get(1, 1), 'B');
    }

    #[test]
    fn resize_expand_width() {
        let mut grid = CharGrid::new(5, 5);

        assert!(!grid.contains(5, 0));

        grid.resize(6, 5);

        assert!(grid.contains(5, 0));
        assert_eq!(grid.width(), 6);
        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn resize_shrink_width() {
        let mut grid = CharGrid::new(5, 5);

        assert!(grid.contains(4, 0));

        grid.resize(4, 5);

        assert!(!grid.contains(4, 0));
        assert_eq!(grid.width(), 4);
        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn resize_expand_height() {
        let mut grid = CharGrid::new(5, 5);

        assert!(!grid.contains(0, 5));

        grid.resize(5, 6);

        assert!(grid.contains(0, 5));
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 6);
    }

    #[test]
    fn resize_shrink_height() {
        let mut grid = CharGrid::new(5, 5);

        assert!(grid.contains(0, 4));

        grid.resize(5, 4);

        assert!(!grid.contains(0, 4));
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 4);
    }

    #[test]
    fn push_x() {
        let mut grid = CharGrid::new(5, 5);

        grid.push_x(0, 'A');

        assert_eq!(grid.get(5, 0), 'A');
        assert_eq!(grid.get(5, 1), '?');
    }

    #[test]
    fn push_y() {
        let mut grid = CharGrid::new(5, 5);

        grid.push_y(0, 'A');

        assert_eq!(grid.get(0, 5), 'A');
        assert_eq!(grid.get(1, 5), '?');
    }

    #[test]
    fn columns_iter() {
        let grid: CharGrid = "ABC
DEF
GHI
JKL"
        .into();

        let expected = vec![
            vec!['A', 'D', 'G', 'J'],
            vec!['B', 'E', 'H', 'K'],
            vec!['C', 'F', 'I', 'L'],
        ];

        let actual: Vec<Vec<char>> = grid.columns_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn position_of() {
        let grid: CharGrid = "ABC
DEF
GHI
JKL"
        .into();

        let actual = grid.position_of('H');
        assert_eq!(actual, Some((1, 2)));
    }

    #[test]
    fn position_of_not_found() {
        let grid: CharGrid = "ABC
DEF
GHI
JKL"
        .into();

        let actual = grid.position_of('M');
        assert!(actual.is_none());
    }
}
