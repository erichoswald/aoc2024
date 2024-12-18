use aoc2024::grid::{Grid, GridPos};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let sample = include_str!("../../inputs/sample18.txt");
    let input = include_str!("../../inputs/day18.txt");

    println!("Sample");
    simulate(sample, 7, 7, 12);
    println!("\nPuzzle");
    simulate(input, 71, 71, 1024);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Path {
    steps: usize,
    pos: GridPos,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

fn simulate(input: &str, row_count: usize, col_count: usize, bytes: usize) {
    let coords = parse_coords(input);
    let mut memory = Grid::new(row_count, col_count);
    corrupt_memory(&mut memory, &coords.as_slice()[..bytes]);
    let start_pos = GridPos::new(0, 0);
    let end_pos = GridPos::new(row_count - 1, col_count - 1);
    let steps = find_shortest_path(&mut memory, start_pos, end_pos).unwrap();
    println!("Number of steps after {bytes} bytes: {steps}");

    let mut bytes = bytes;
    loop {
        memory.set(coords[bytes], '#');
        if find_shortest_path(&memory, start_pos, end_pos).is_none() {
            break;
        }
        bytes += 1;
    }
    let last_pos = coords[bytes];
    println!("End point unreachable after {bytes}, last bytes dropped at {},{}", last_pos.col(), last_pos.row());
}

fn corrupt_memory(memory: &mut Grid<char>, coords: &[GridPos]) {
    for coord in coords {
        memory.set(*coord, '#')
    }
}

fn find_shortest_path(memory: &Grid<char>, start_pos: GridPos, end_pos: GridPos) -> Option<usize> {
    let mut visited = HashSet::from([ start_pos ]);
    let mut queue = BinaryHeap::new();
    queue.push(Path { steps: 0, pos: start_pos });
    while let Some(Path { steps, pos }) = queue.pop() {
        if pos == end_pos {
            return Some(steps);
        }
        for neighbour in pos.neighbours_4() {
            if memory.is_inside(neighbour) && !memory.is_defined(neighbour) && visited.insert(neighbour) {
                queue.push(Path { steps: steps + 1, pos: neighbour });
            }
        }
    }
    None
}

fn parse_coords(input: &str) -> Vec<GridPos> {
    input.lines().map(|line| {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        GridPos::new(y, x)
    }).collect()
}
