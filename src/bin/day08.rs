use std::collections::{HashMap, HashSet};

fn main() {
    let sample = include_str!("../../inputs/sample08.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day08.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(sample: &str) -> usize {
    let grid = Grid::parse_from(sample);
    let mut anti_nodes = HashSet::new();
    let frequencies = grid.cell_values('.');
    for frequency in frequencies {
        let nodes = grid.cell_coords_with(frequency);
        for a0 in &nodes {
            for a1 in &nodes {
                if a0 != a1 {
                    let dist = a0.distance_to(&a1);
                    let candidate = a1.add(&dist);
                    if grid.contains(&candidate) {
                        anti_nodes.insert(candidate);
                    }
                }
            }
        }
    }
    anti_nodes.len()
}

fn part2(sample: &str) -> usize {
    let grid = Grid::parse_from(sample);
    let mut anti_nodes = HashSet::new();
    let frequencies = grid.cell_values('.');
    for frequency in frequencies {
        let nodes = grid.cell_coords_with(frequency);
        for a0 in &nodes {
            for a1 in &nodes {
                if a0 != a1 {
                    let dist = a0.distance_to(&a1);
                    let mut candidate = GridCoord::at(a1);
                    while grid.contains(&candidate) {
                        anti_nodes.insert(GridCoord::at(&candidate));
                        candidate = candidate.add(&dist);
                    }
                }
            }
        }
    }
    anti_nodes.len()
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct GridCoord(usize, usize);

impl GridCoord {
    fn distance_to(&self, other: &GridCoord) -> GridDist {
        GridDist(
            other.0 as i32 - self.0 as i32,
            other.1 as i32 - self.1 as i32,
        )
    }

    fn add(&self, dist: &GridDist) -> GridCoord {
        GridCoord(
            (self.0 as i32 + dist.0) as usize,
            (self.1 as i32 + dist.1) as usize,
        )
    }

    fn at(coord: &GridCoord) -> GridCoord {
        GridCoord(coord.0, coord.1)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct GridDist(i32, i32);

impl GridDist {
    fn add(&self, dist: &GridDist) -> GridDist {
        GridDist(self.0 + dist.0, self.1 + dist.1)
    }

    fn mult(&self, factor: i32) -> GridDist {
        GridDist(self.0 * factor, self.1 * factor)
    }
}

struct Grid {
    cells: HashMap<GridCoord, char>,
}

impl Grid {
    fn cell_values(&self, ignoring: char) -> HashSet<char> {
        let mut seen = HashSet::new();
        self.cells.values().for_each(|c| {
            seen.insert(*c);
        });
        seen.remove(&ignoring);
        seen
    }

    fn cell_coords_with(&self, cell: char) -> HashSet<&GridCoord> {
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

    fn contains(&self, coord: &GridCoord) -> bool {
        self.cells.contains_key(coord)
    }

    fn parse_from(input: &str) -> Self {
        let mut cells = HashMap::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, ch)| {
                cells.insert(GridCoord(row, col), ch);
            })
        });
        Grid { cells }
    }
}
