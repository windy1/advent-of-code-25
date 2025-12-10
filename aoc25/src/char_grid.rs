use std::fmt;
use std::fmt::{Display, Formatter};

pub struct CharGrid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
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

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
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

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.data.iter().flat_map(|row| row.iter())
    }
}

impl Clone for CharGrid {
    fn clone(&self) -> Self {
        let mut new_grid = CharGrid::new(self.width(), self.height());

        for y in 0..self.height() {
            for x in 0..self.width() {
                new_grid.set(x, y, self.get(x, y));
            }
        }

        new_grid
    }
}

impl Display for CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for c in row {
                write!(f, "{:>2}", c)?;
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
}
