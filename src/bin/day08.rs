use aoc2024::grid::{Grid, GridPos};
use std::collections::HashSet;

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
    let frequencies = grid.cell_values_ignoring('.');
    for frequency in frequencies {
        let nodes = grid.cell_positions_with(frequency);
        for a0 in &nodes {
            for a1 in &nodes {
                if a0 != a1 {
                    let dist = a0.move_to(&a1);
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
    let frequencies = grid.cell_values_ignoring('.');
    for frequency in frequencies {
        let nodes = grid.cell_positions_with(frequency);
        for a0 in &nodes {
            for a1 in &nodes {
                if a0 != a1 {
                    let dist = a0.move_to(&a1);
                    let mut candidate = GridPos::at(a1);
                    while grid.contains(&candidate) {
                        anti_nodes.insert(GridPos::at(&candidate));
                        candidate = candidate.add(&dist);
                    }
                }
            }
        }
    }
    anti_nodes.len()
}
