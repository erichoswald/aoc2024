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
    let mut possible_count = 0;
    let mut evaluated_designs = HashMap::new();
    for design in &designs {
        if is_design_possible(design, patterns.as_slice(), &mut evaluated_designs) {
            possible_count += 1;
        }
    }
    println!("Possible designs: {possible_count}");
}

fn is_design_possible<'a>(design: &'a str, patterns: &[&str], evaluated_designs: &mut HashMap<&'a str, bool>) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(is_possible) = evaluated_designs.get(design) {
        return *is_possible;
    }
    for pattern in patterns {
        if design.starts_with(*pattern) {
            let is_possible = is_design_possible(&design[pattern.len()..], patterns, evaluated_designs);
            evaluated_designs.insert(design, is_possible);
            if is_possible {
                return true;
            }
        }
    }
    false
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect();
    lines.next();
    let designs = lines.collect();
    (patterns, designs)
}
