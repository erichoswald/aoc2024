use std::collections::HashMap;
use std::ops::Add;
use std::vec;
use itertools::Itertools;
use aoc2024::grid::{Grid, GridPos, EAST, NORTH, SOUTH, WEST};

fn main() {
    let sample = include_str!("../../inputs/sample21.txt");
    println!("Part 1 (Sample)");
    solve(&sample, 2);

    let input = include_str!("../../inputs/day21.txt");
    println!("Part 1");
    solve(&input, 2);
    println!("Part 2");
    solve(&input, 25);
}

fn solve(input: &str, indirections: usize) {
    let mut total_complexity = 0;
    for line in input.lines() {
        let shortest_sequence_length = find_shortest_sequence_length(line, indirections);
        total_complexity += shortest_sequence_length * numeric_part(line);
    }
    println!("Total complexity: {}", total_complexity);
}

fn numeric_part(line: &str) -> usize {
    line.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<usize>().unwrap()
}

fn find_shortest_sequence_length(keys: &str, indirections: usize) -> usize {
    let numeric_keypad = new_numeric_keypad();
    let directional_keypad = new_directional_keypad();
    let mut cache = HashMap::new();
    find_sequences(&numeric_keypad, keys).iter()
        .map(|sequence| find_shortest_directional_sequence_length(&directional_keypad, indirections, sequence, &mut cache))
        .min()
        .unwrap()
}

fn find_shortest_directional_sequence_length(keypad: &Grid<char>, indirections: usize, sequence: &Vec<String>, cache: &mut HashMap<String, usize>) -> usize {
    if indirections == 0 {
        sequence.iter().map(|s| s.len()).sum()
    } else {
        sequence.iter().map(|keys| {
            let cache_key = format!("{indirections}:{keys}");
            if let Some(cached) = cache.get(&cache_key) {
                *cached
            } else {
                let computed = find_sequences(keypad, keys).iter()
                    .map(|s| find_shortest_directional_sequence_length(keypad, indirections - 1, s, cache))
                    .min()
                    .unwrap();
                cache.insert(cache_key, computed);
                computed
            }
        }).sum()
    }
}

fn find_sequences(keypad: &Grid<char>, keys: &str) -> Vec<Vec<String>> {
    let mut sequences = Vec::new();
    let mut from = 'A';
    for to in keys.chars() {
        let segments = find_shortest_segments(keypad, from, to);
        sequences.push(segments);
        from = to;
    }
    combine_sequences(&sequences)
}

fn find_shortest_segments(keypad: &Grid<char>, from: char, to: char) -> Vec<String> {
    let from_pos = keypad.position_of(from);
    let to_pos = keypad.position_of(to);
    let candidates = vec![(String::new(), from_pos)];
    let mut segments = Vec::new();
    append_shortest_segments(keypad, to_pos, &candidates, &mut segments);
    segments
}

fn append_shortest_segments(keypad: &Grid<char>, to_pos: GridPos, candidates: &Vec<(String, GridPos)>, segments: &mut Vec<String>) {
    let mut finishers: Vec<String> = Vec::new();
    for (segment, pos) in candidates {
        if *pos == to_pos {
            let mut s = String::from(segment);
            s.push('A');
            finishers.push(s);
        }
    }
    if !finishers.is_empty() {
        segments.extend(finishers);
    } else {
        let mut next_candidates = Vec::new();
        for (segment, pos) in candidates {
            add_candidates(keypad, '^', pos.add(NORTH), segment, &mut next_candidates);
            add_candidates(keypad, 'v', pos.add(SOUTH), segment, &mut next_candidates);
            add_candidates(keypad, '<', pos.add(WEST), segment, &mut next_candidates);
            add_candidates(keypad, '>', pos.add(EAST), segment, &mut next_candidates);
        }
        append_shortest_segments(keypad, to_pos, &next_candidates, segments);
    }
}

fn add_candidates(keypad: &Grid<char>, ch: char, next_pos: GridPos, segment: &String, next_candidates: &mut Vec<(String, GridPos)>) {
    if keypad.is_defined(next_pos) {
        let mut next_segment = String::from(segment);
        next_segment.push(ch);
        next_candidates.push((next_segment, next_pos));
    }
}

fn combine_sequences<T : Clone>(sequences: &[Vec<T>]) -> Vec<Vec<T>> {
    match sequences {
        [first] => {
            first.iter().map(|s| vec![s.clone()]).collect()
        },
        [first, ..] => {
            let mut result = Vec::new();
            let tail = combine_sequences(&sequences[1..]);
            for a in first {
                for b in &tail {
                    let mut s = vec![a.clone()];
                    s.extend(b.clone());
                    result.push(s);
                }
            }
            result
        },
        _ => vec![]
    }
}

fn new_numeric_keypad() -> Grid<char> {
    let mut keypad = Grid::new(4, 3);
    keypad.set(GridPos::new(0, 0), '7');
    keypad.set(GridPos::new(0, 1), '8');
    keypad.set(GridPos::new(0, 2), '9');
    keypad.set(GridPos::new(1, 0), '4');
    keypad.set(GridPos::new(1, 1), '5');
    keypad.set(GridPos::new(1, 2), '6');
    keypad.set(GridPos::new(2, 0), '1');
    keypad.set(GridPos::new(2, 1), '2');
    keypad.set(GridPos::new(2, 2), '3');
    keypad.set(GridPos::new(3, 1), '0');
    keypad.set(GridPos::new(3, 2), 'A');
    keypad
}

fn new_directional_keypad() -> Grid<char> {
    let mut keypad = Grid::new(2, 3);
    keypad.set(GridPos::new(0, 1), '^');
    keypad.set(GridPos::new(0, 2), 'A');
    keypad.set(GridPos::new(1, 0), '<');
    keypad.set(GridPos::new(1, 1), 'v');
    keypad.set(GridPos::new(1, 2), '>');
    keypad
}
