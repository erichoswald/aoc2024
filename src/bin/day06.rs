use aoc2024::grid::{Grid, NORTH};
use std::collections::HashSet;

fn main() {
    let sample = include_str!("../../inputs/sample06.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day06.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut obstacles = Grid::parse_and_map_from(input, |ch| {
        match ch {
            '#' | '^' => Some(ch),
            _ => None,
        }
    });
    let mut pos = *obstacles.cell_positions_with('^').iter().next().unwrap();
    obstacles.remove(pos);
    let mut visited = HashSet::from([ pos ]);
    let mut direction = NORTH;
    loop {
        let next_pos = pos.add(direction);
        if !obstacles.is_inside(next_pos) {
            break;
        }
        if obstacles.is_defined(next_pos) {
            direction = direction.turn_right();
        } else {
            visited.insert(next_pos);
            pos = next_pos;
        }
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    0
}
