use aoc2024::grid::{Grid, GridPos};
use std::collections::HashSet;

fn main() {
    let sample = include_str!("../../inputs/sample12.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day12.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let map = Grid::parse_from(input);
    let mut extracted: HashSet<GridPos> = HashSet::new();
    let mut total_price = 0;
    for row in 0..map.row_count() {
        for col in 0..map.col_count() {
            let pos = GridPos::new(row, col);
            if !extracted.contains(&pos) {
                let plant_type = map.at(pos).unwrap();
                let mut visited = HashSet::new();
                let mut area = 0;
                let mut perimeter = 0;
                visit_region(&map, pos, plant_type, &mut visited, &mut area, &mut perimeter);
                extracted.extend(&visited);
                total_price += area * perimeter;
            }
        }
    }
    total_price
}

fn part2(input: &str) -> usize {
    0
}

fn visit_region(
    map: &Grid<char>,
    pos: GridPos,
    plant_type: char,
    visited: &mut HashSet<GridPos>,
    area: &mut u32,
    perimeter: &mut u32,
) {
    if map.at(pos) == Some(plant_type) && visited.insert(pos) {
        *area += 1;
        let mut perimeter_count = 4;
        for neighbour in pos.neighbours_4() {
            visit_region(map, neighbour, plant_type, visited, area, perimeter);
            if visited.contains(&neighbour) {
                perimeter_count -= 1;
            }
        }
        *perimeter += perimeter_count;
    }
}
