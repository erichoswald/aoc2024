use std::cmp::{max, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Iter;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

pub static NORTH: GridMove = GridMove(-1, 0);
pub static SOUTH: GridMove = GridMove(1, 0);
pub static WEST: GridMove = GridMove(0, -1);
pub static EAST: GridMove = GridMove(0, 1);
pub static NORTH_WEST: GridMove = GridMove(-1, -1);
pub static NORTH_EAST: GridMove = GridMove(-1, 1);
pub static SOUTH_WEST: GridMove = GridMove(1, -1);
pub static SOUTH_EAST: GridMove = GridMove(1, 1);

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct GridMove(i32, i32);

impl GridMove {
    pub fn new(dy: i32, dx: i32) -> GridMove {
        GridMove(dy, dx)
    }

    pub fn distance(&self) -> usize {
        self.0.abs() as usize + self.1.abs() as usize
    }

    pub fn turn_left(&self) -> GridMove {
        GridMove(-self.1, self.0)
    }

    pub fn turn_right(&self) -> GridMove {
        GridMove(self.1, -self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct GridPos(usize, usize);

impl GridPos {
    pub fn new(row: usize, col: usize) -> GridPos {
        GridPos(row, col)
    }

    pub fn row(&self) -> usize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1
    }

    pub fn move_to(&self, other: GridPos) -> GridMove {
        GridMove(
            other.0 as i32 - self.0 as i32,
            other.1 as i32 - self.1 as i32,
        )
    }

    pub fn add(&self, dist: GridMove) -> GridPos {
        GridPos(
            (self.0 as i32 + dist.0) as usize,
            (self.1 as i32 + dist.1) as usize,
        )
    }

    pub fn neighbours_4(&self) -> Vec<GridPos> {
        vec![
            self.add(NORTH),
            self.add(SOUTH),
            self.add(WEST),
            self.add(EAST),
        ]
    }

    pub fn compare(&self, other: &GridPos) -> Ordering {
        if self.0 < other.0 {
            Less
        } else if self.0 > other.0 {
            Greater
        } else if self.1 < other.1 {
            Less
        } else if self.1 > other.1 {
            Greater
        } else {
            Equal
        }
    }
}

pub struct Grid<T : Copy> {
    row_count: usize,
    col_count: usize,
    cells: HashMap<GridPos, T>,
}

impl <T : Copy> Grid<T> {
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn col_count(&self) -> usize {
        self.col_count
    }

    pub fn is_inside(&self, pos: GridPos) -> bool {
        pos.0 < self.row_count && pos.1 < self.col_count
    }

    pub fn is_defined(&self, coord: GridPos) -> bool {
        self.cells.contains_key(&coord)
    }

    pub fn iter(&self) -> Iter<'_, GridPos, T> {
        self.cells.iter()
    }

    pub fn at(&self, coord: GridPos) -> Option<T> {
        self.cells.get(&coord).map(|&t| t)
    }

    pub fn set(&mut self, pos: GridPos, value: T) {
        self.cells.insert(pos, value);
    }

    pub fn remove(&mut self, pos: GridPos) {
        self.cells.remove(&pos);
    }

    pub fn new(row_count: usize, col_count: usize) -> Grid<T> {
        Grid { row_count, col_count, cells: HashMap::new() }
    }

    pub fn parse_and_map_from<F : Fn(char) -> Option<T>>(input: &str, transform: F) -> Grid<T> {
        let mut max_row = 0;
        let mut max_col = 0;
        let mut cells: HashMap<GridPos, T> = HashMap::new();
        input.lines().enumerate().for_each(|(row, line)| {
            max_row = max(max_row, row);
            line.chars().enumerate().for_each(|(col, ch)| {
                max_col = max(max_col, col);
                let cell = transform(ch);
                if cell.is_some() {
                    cells.insert(GridPos(row, col), cell.unwrap());
                }
            })
        });
        Grid { row_count: max_row + 1, col_count: max_col + 1, cells }
    }
}

impl <T : Copy + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.row_count() {
            for col in 0..self.col_count() {
                let pos = GridPos(row, col);
                match self.cells.get(&pos) {
                    Some(cell) => write!(f, "{}", *cell)?,
                    None => write!(f, ".")?
                }
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

    pub fn cell_positions_having<F : Fn(T) -> bool>(&self, predicate: F) -> HashSet<GridPos> {
        let mut coords = HashSet::new();
        self.cells
            .keys()
            .for_each(|coord| {
                if predicate(self.cells[coord]) {
                    coords.insert(*coord);
                }
            });
        coords
    }

    pub fn cell_positions_with(&self, cell: T) -> HashSet<GridPos> {
        self.cell_positions_having(|c| cell == c)
    }

    pub fn position_of(&self, cell: T) -> GridPos {
        *self.cell_positions_with(cell).iter().next().unwrap()
    }
}

impl Grid<char> {
    pub fn parse_from(input: &str) -> Grid<char> {
        Self::parse_and_map_from(input, |ch| Some(ch))
    }
}
