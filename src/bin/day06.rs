use aoc2024::grid::{Grid, GridMove, GridPos, NORTH};
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
    let (obstacles, mut pos) = parse_obstacles(input);
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
    let (mut obstacles, start) = parse_obstacles(input);
    let mut loop_count = 0;
    for row in 0..obstacles.row_count() {
        for col in 0..obstacles.col_count() {
            let obstacle_pos = GridPos::new(row, col);
            if obstacle_pos != start && !obstacles.is_defined(obstacle_pos) {
                obstacles.set(obstacle_pos, 'O');
                if is_loop(&obstacles, start, NORTH) {
                    loop_count += 1;
                }
                obstacles.remove(obstacle_pos);
            }
        }
    }
    loop_count
}

fn is_loop(obstacles: &Grid<char>, start: GridPos, mut direction: GridMove) -> bool {
    let mut visited = HashSet::from([ (start, direction) ]);
    let mut pos = start;
    loop {
        let next_pos = pos.add(direction);
        if !obstacles.is_inside(next_pos) {
            return false;
        } else if obstacles.is_defined(next_pos) {
            direction = direction.turn_right();
        } else {
            pos = next_pos;
            if !visited.insert((pos, direction)) {
                return true;
            }
        }
    }
}

fn parse_obstacles(input: &str) -> (Grid<char>, GridPos) {
    let mut obstacles = Grid::parse_and_map_from(input, |ch| {
        match ch {
            '#' | '^' => Some(ch),
            _ => None,
        }
    });
    let pos = *obstacles.cell_positions_with('^').iter().next().unwrap();
    obstacles.remove(pos);
    (obstacles, pos)
}
