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
    move_robot(&mut grid, moves)
}

fn part2(input: &str) -> usize {
    let (grid, moves) = parse_input(input);
    let mut grid = widen_grid(&grid);
    move_robot(&mut grid, moves)
}

fn move_robot(grid: &mut Grid<char>, moves: Vec<GridMove>) -> usize {
    let mut robot_pos = *grid.cell_positions_with('@').iter().next().unwrap();
    for robot_move in moves {
        let next_pos = robot_pos.add(robot_move);
        if make_room(grid, next_pos, robot_move, true) {
            make_room(grid, next_pos, robot_move, false);
            grid.set(next_pos, '@');
            grid.remove(robot_pos);
            robot_pos = next_pos;
        }
    }
    grid.cell_positions_having(|ch| ch == 'O' || ch == '[')
        .iter()
        .fold(0, |acc, box_pos| acc + 100 * box_pos.row() + box_pos.col())
}

fn make_room(grid: &mut Grid<char>, pos: GridPos, direction: GridMove, probe_only: bool) -> bool {
    match grid.at(pos) {
        Some('#') => false,
        Some('O') => {
            let next_pos = pos.add(direction);
            if make_room(grid, next_pos, direction, probe_only) {
                if !probe_only {
                    grid.set(next_pos, 'O');
                }
                true
            } else {
                false
            }
        }
        Some('[') => {
            let next_pos = pos.add(direction);
            let make_room_north_south = (direction == NORTH || direction == SOUTH)
                && make_room(grid, next_pos, direction, probe_only)
                && make_room(grid, next_pos.add(EAST), direction, probe_only);
            let make_room_east = direction == EAST
                && make_room(grid, next_pos.add(EAST), direction, probe_only);
            if make_room_north_south || make_room_east {
                if !probe_only {
                    grid.remove(pos.add(EAST));
                    grid.set(next_pos, '[');
                    grid.set(next_pos.add(EAST), ']');
                }
                true
            } else {
                false
            }
        }
        Some(']') => {
            let next_pos = pos.add(direction);
            let make_room_north_south = (direction == NORTH || direction == SOUTH)
                && make_room(grid, next_pos, direction, probe_only)
                && make_room(grid, next_pos.add(WEST), direction, probe_only);
            let make_room_west = direction == WEST
                && make_room(grid, next_pos.add(WEST), direction, probe_only);
            if make_room_north_south || make_room_west {
                if !probe_only {
                    grid.remove(pos.add(WEST));
                    grid.set(next_pos, ']');
                    grid.set(next_pos.add(WEST), '[');
                }
                true
            } else {
                false
            }
        }
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

fn widen_grid(source: &Grid<char>) -> Grid<char> {
    let mut target = Grid::new(source.row_count(), 2 * source.col_count());
    source.iter().for_each(|(pos, ch)| {
        let first_pos = GridPos::new(pos.row(), 2 * pos.col());
        let second_pos = first_pos.add(EAST);
        match ch {
            '#' => {
                target.set(first_pos, '#');
                target.set(second_pos, '#');
            }
            'O' => {
                target.set(first_pos, '[');
                target.set(second_pos, ']');
            }
            '@' => target.set(first_pos, '@'),
            _ => {}
        }
    });
    target
}
