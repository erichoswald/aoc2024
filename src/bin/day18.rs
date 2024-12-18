use aoc2024::grid::{Grid, GridPos};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let sample = include_str!("../../inputs/sample18.txt");
    let input = include_str!("../../inputs/day18.txt");

    println!("Part1 (sample): {}", part1(sample, 7, 7, 12));
    println!("Part1: {}", part1(input, 71, 71, 1024));
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

fn part1(input: &str, row_count: usize, col_count: usize, bytes: usize) -> usize {
    let coords = parse_coords(input);
    let mut memory = Grid::new(row_count, col_count);
    corrupt_memory(&mut memory, &coords.as_slice()[..bytes]);
    find_shortest_path(&mut memory, GridPos::new(0, 0), GridPos::new(row_count - 1, col_count - 1))
}

fn corrupt_memory(memory: &mut Grid<char>, coords: &[GridPos]) {
    for coord in coords {
        memory.set(*coord, '#')
    }
}

fn find_shortest_path(memory: &mut Grid<char>, start_pos: GridPos, end_pos: GridPos) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Path { steps: 0, pos: start_pos });
    memory.set(start_pos, 'O');
    while let Some(Path { steps, pos }) = queue.pop() {
        if pos == end_pos {
            return steps;
        }
        for neighbour in pos.neighbours_4() {
            if memory.is_inside(neighbour) && memory.at(neighbour) == None {
                memory.set(neighbour, 'O');
                queue.push(Path { steps: steps + 1, pos: neighbour });
            }
        }
    }
    0
}

fn parse_coords(input: &str) -> Vec<GridPos> {
    input.lines().map(|line| {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        GridPos::new(y, x)
    }).collect()
}
