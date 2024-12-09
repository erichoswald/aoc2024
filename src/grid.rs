use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct GridPos(usize, usize);

impl GridPos {
    pub fn move_to(&self, other: &GridPos) -> GridMove {
        GridMove(
            other.0 as i32 - self.0 as i32,
            other.1 as i32 - self.1 as i32,
        )
    }

    pub fn add(&self, dist: &GridMove) -> GridPos {
        GridPos(
            (self.0 as i32 + dist.0) as usize,
            (self.1 as i32 + dist.1) as usize,
        )
    }

    pub fn at(coord: &GridPos) -> GridPos {
        GridPos(coord.0, coord.1)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct GridMove(i32, i32);

impl GridMove {
    pub fn add(&self, dist: &GridMove) -> GridMove {
        GridMove(self.0 + dist.0, self.1 + dist.1)
    }

    pub fn mult(&self, factor: i32) -> GridMove {
        GridMove(self.0 * factor, self.1 * factor)
    }
}

pub struct Grid {
    cells: HashMap<GridPos, char>,
}

impl Grid {
    pub fn cell_values_ignoring(&self, ignoring: char) -> HashSet<char> {
        let mut seen = HashSet::new();
        self.cells.values().for_each(|c| {
            seen.insert(*c);
        });
        seen.remove(&ignoring);
        seen
    }

    pub fn cell_positions_with(&self, cell: char) -> HashSet<&GridPos> {
        let mut coords = HashSet::new();
        self.cells
            .keys()
            .for_each(|coord| {
                if self.cells[coord] == cell {
                    coords.insert(coord);
                }
            });
        coords
    }

    pub fn contains(&self, coord: &GridPos) -> bool {
        self.cells.contains_key(coord)
    }

    pub fn parse_from(input: &str) -> Self {
        let mut cells = HashMap::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                cells.insert(GridPos(row, col), ch);
            })
        });
        Grid { cells }
    }
}
