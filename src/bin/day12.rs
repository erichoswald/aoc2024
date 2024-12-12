use aoc2024::grid::{Grid, GridPos, EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_WEST, WEST};
use std::collections::HashSet;

fn main() {
    let sample = include_str!("../../inputs/sample12.txt");
    let (part1, part2) = parts(sample);
    println!("Part 1 (sample): {}", part1);
    println!("Part 2 (sample): {}\n", part2);

    let input = include_str!("../../inputs/day12.txt");
    let (part1, part2) = parts(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parts(input: &str) -> (usize, usize) {
    let map = Grid::parse_from(input);
    let mut extracted: HashSet<GridPos> = HashSet::new();
    let mut total_perimeter_price = 0;
    let mut total_edges_price = 0;
    for row in 0..map.row_count() {
        for col in 0..map.col_count() {
            let pos = GridPos::new(row, col);
            if !extracted.contains(&pos) {
                let plant_type = map.at(pos).unwrap();
                let mut region = HashSet::new();
                let mut perimeter = 0;
                extract_region(&map, pos, plant_type, &mut region, &mut perimeter);
                let area = region.len();
                total_perimeter_price += area * perimeter;
                let edge_count = count_edges(&region);
                total_edges_price += area * edge_count;
                extracted.extend(&region);
            }
        }
    }
    (total_perimeter_price, total_edges_price)
}

fn extract_region(
    map: &Grid<char>,
    pos: GridPos,
    plant_type: char,
    region: &mut HashSet<GridPos>,
    perimeter: &mut usize,
) {
    if map.at(pos) == Some(plant_type) && region.insert(pos) {
        let mut perimeter_count = 4;
        for neighbour in pos.neighbours_4() {
            extract_region(map, neighbour, plant_type, region, perimeter);
            if region.contains(&neighbour) {
                perimeter_count -= 1;
            }
        }
        *perimeter += perimeter_count;
    }
}

fn count_edges(region: &HashSet<GridPos>) -> usize {
    let mut edge_count = 0;
    for plot in region {
        let north = region.contains(&plot.add(NORTH));
        let south = region.contains(&plot.add(SOUTH));
        let west = region.contains(&plot.add(WEST));
        let east = region.contains(&plot.add(EAST));
        let north_west = region.contains(&plot.add(NORTH_WEST));
        let north_east = region.contains(&plot.add(NORTH_EAST));
        let south_west = region.contains(&plot.add(SOUTH_WEST));
        if !north && (!west || north_west) {
            edge_count += 1; // Plot starts a new edge on north side.
        }
        if !south && (!west || south_west) {
            edge_count += 1; // Plot starts a new edge on south side.
        }
        if !west && (!north || north_west) {
            edge_count += 1; // Plot starts a new edge on west side.
        }
        if !east && (!north || north_east) {
            edge_count += 1; // Plot starts a new edge on east side.
        }
    }
    edge_count
}
