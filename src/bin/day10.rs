use aoc2024::grid::{Grid, GridPos};
use std::collections::HashSet;

fn main() {
    let sample = include_str!("../../inputs/sample10.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day10.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::parse_and_map_from(input, |ch| ch.to_digit(10).unwrap());
    let mut score = 0;
    let trailheads = grid.cell_positions_with(0);
    for trailhead in trailheads {
        let mut peaks = HashSet::new();
        mark_peaks(&grid, trailhead, 0, &mut peaks);
        score += peaks.len();
    }
    score
}

fn part2(input: &str) -> usize {
    let grid = Grid::parse_and_map_from(input, |ch| ch.to_digit(10).unwrap());
    let mut ratings = 0;
    let trailheads = grid.cell_positions_with(0);
    for trailhead in trailheads {
        ratings += rating(&grid, trailhead, 0);
    }
    ratings
}

fn mark_peaks(grid: &Grid<u32>, grid_pos: &GridPos, height: u32, peaks: &mut HashSet<GridPos>) {
    if *grid.at(grid_pos).unwrap() == height {
        if height < 9 {
            grid_pos
                .neighbours_4()
                .iter()
                .filter(|neighbour| grid.contains(&neighbour))
                .for_each(|neighbour| {
                    mark_peaks(grid, neighbour, height + 1, peaks)
                })
        } else {
            peaks.insert(GridPos::at(grid_pos));
        }
    }
}

fn rating(grid: &Grid<u32>, grid_pos: &GridPos, height: u32) -> usize {
    if *grid.at(grid_pos).unwrap() != height {
        0
    } else if height < 9 {
        grid_pos
            .neighbours_4()
            .iter()
            .filter(|neighbour| grid.contains(&neighbour))
            .fold(0, |sum, neighbour| {
                sum + rating(grid, neighbour, height + 1)
            })
    } else {
        1
    }
}
