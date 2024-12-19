use std::collections::HashMap;

fn main() {
    let sample = include_str!("../../inputs/sample19.txt");
    println!("Sample");
    solve_puzzle(&sample);

    let input = include_str!("../../inputs/day19.txt");
    println!("Puzzle");
    solve_puzzle(&input);
}

fn solve_puzzle(input: &str) {
    let (mut patterns, designs) = parse_input(input);
    patterns.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    let mut possible_design_count = 0;
    let mut possible_ways_count = 0;
    for design in &designs {
        let possible_designs = count_possible_designs(design, patterns.as_slice(), &mut HashMap::new());
        if possible_designs != 0 {
            possible_design_count += 1;
        }
        possible_ways_count += possible_designs;
    }
    println!("Possible designs: {possible_design_count}");
    println!("Possible ways: {possible_ways_count}");
}

fn count_possible_designs<'a>(design: &'a str, patterns: &[&str], evaluated_segments: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(possible_designs) = evaluated_segments.get(design) {
        return *possible_designs;
    }
    let mut count = 0;
    for pattern in patterns {
        if design.starts_with(*pattern) {
            count += count_possible_designs(&design[pattern.len()..], patterns, evaluated_segments);
        }
        evaluated_segments.insert(design, count);
    }
    count
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect();
    lines.next();
    let designs = lines.collect();
    (patterns, designs)
}
