use std::fmt::{self, Display, Formatter};

use crate::char_grid::CharGrid;

// TODO: wip

pub struct Slice<'a> {
    grid: &'a CharGrid,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl<'a> Slice<'a> {
    pub fn new(grid: &'a CharGrid, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            grid,
            x,
            y,
            width,
            height,
        }
    }
}

impl Display for Slice<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let grid = self.grid;
        let cell_width = grid.cell_width;

        if grid.axes_enabled {
            write!(f, "{:>cell_width$}", " ")?;
            for x in self.x..self.x + self.width {
                write!(f, "{:>cell_width$}", x.to_string())?;
            }
            writeln!(f)?;
        }

        for y in self.y..self.y + self.height {
            if grid.axes_enabled {
                write!(f, "{:>cell_width$}", y.to_string())?;
            }

            for x in self.x..self.x + self.width {
                write!(f, "{:>cell_width$}", grid.get(x, y))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
