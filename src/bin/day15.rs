use std::fmt::{Display, Formatter};
use aoc2024::grid::{Grid, GridMove, GridPos, EAST, NORTH, SOUTH, WEST};

fn main() {
    let sample = include_str!("../../inputs/sample15.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day15.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (mut grid, moves) = parse_input(input);
    let mut robot_pos = *grid.cell_positions_with('@').iter().next().unwrap();
    for robot_move in moves {
        let next_pos = robot_pos.add(robot_move);
        if make_room(&mut grid, next_pos, robot_move) {
            grid.set(next_pos, '@');
            grid.remove(robot_pos);
            robot_pos = next_pos;
        }
    }
    grid.cell_positions_with('O').iter().fold(0, |acc, box_pos| {
        acc + 100 * box_pos.row() + box_pos.col()
    })
}

fn part2(input: &str) -> usize {
    0
}

fn make_room(grid: &mut Grid<char>, pos: GridPos, direction: GridMove) -> bool {
    match grid.at(pos) {
        Some('#') => false,
        Some('O') => {
            let next_pos = pos.add(direction);
            if make_room(grid, next_pos, direction) {
                grid.set(next_pos, 'O');
                true
            } else {
                false
            }
        },
        _ => true,
    }
}

fn parse_input(input: &str) -> (Grid<char>, Vec<GridMove>) {
    let divider = input.find("#\n\n").unwrap();
    let grid = Grid::parse_from(&input[..divider + 2]);
    let mut moves = Vec::new();
    for line in input[divider + 3..].lines() {
        for ch in line.chars() {
            match ch {
                '^' => moves.push(NORTH),
                'v' => moves.push(SOUTH),
                '>' => moves.push(EAST),
                '<' => moves.push(WEST),
                _ => println!("Unknown character {}", ch),
            };
        }
    }
    (grid, moves)
}
