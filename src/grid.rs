use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

static NORTH: GridMove = GridMove(-1, 0);
static SOUTH: GridMove = GridMove(1, 0);
static WEST: GridMove = GridMove(0, -1);
static EAST: GridMove = GridMove(0, 1);

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

    pub fn neighbours_4(&self) -> Vec<GridPos> {
        vec![
            self.add(&NORTH),
            self.add(&SOUTH),
            self.add(&WEST),
            self.add(&EAST),
        ]
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

pub struct Grid<T : Copy> {
    cells: HashMap<GridPos, T>,
}

impl <T : Copy> Grid<T> {
    pub fn rows(&self) -> usize {
        self.cells.keys().map(|pos| pos.0).max().unwrap() + 1
    }

    pub fn columns(&self) -> usize {
        self.cells.keys().map(|pos| pos.1).max().unwrap() + 1
    }

    pub fn contains(&self, coord: &GridPos) -> bool {
        self.cells.contains_key(coord)
    }

    pub fn at(&self, coord: &GridPos) -> Option<&T> {
        self.cells.get(coord)
    }

    pub fn parse_and_map_from<F : Fn(char) -> T>(input: &str, transform: F) -> Grid<T> {
        let mut cells: HashMap<GridPos, T> = HashMap::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                cells.insert(GridPos(row, col), transform(ch));
            })
        });
        Grid { cells }
    }
}

impl <T : Copy + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows() {
            for col in 0..self.columns() {
                let pos = GridPos(row, col);
                write!(f, "{}", self.cells.get(&pos).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T : Copy + Eq + Hash> Grid<T> {
    pub fn cell_values_ignoring(&self, ignoring: T) -> HashSet<T> {
        let mut seen = HashSet::new();
        self.cells.values().for_each(|c| {
            seen.insert(*c);
        });
        seen.remove(&ignoring);
        seen
    }

    pub fn cell_positions_having<F : Fn(T) -> bool>(&self, predicate: F) -> HashSet<&GridPos> {
        let mut coords = HashSet::new();
        self.cells
            .keys()
            .for_each(|coord| {
                if predicate(self.cells[coord]) {
                    coords.insert(coord);
                }
            });
        coords
    }

    pub fn cell_positions_with(&self, cell: T) -> HashSet<&GridPos> {
        self.cell_positions_having(|c| cell == c)
    }
}

impl Grid<char> {
    pub fn parse_from(input: &str) -> Grid<char> {
        let mut cells = HashMap::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                cells.insert(GridPos(row, col), ch);
            })
        });
        Grid { cells }
    }
}
