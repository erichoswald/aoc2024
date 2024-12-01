use std::collections::HashMap;

fn main() {
    let sample = include_str!("../../inputs/sample01.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day01.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let mut appearance_counts = HashMap::new();
    for id in right.iter() {
        let entry = appearance_counts.entry(id).or_insert(0);
        *entry += 1;
    }

    left.iter().fold(0, |acc, id| {
        let appearance_count = appearance_counts.get(&id).unwrap_or(&0);
        acc + id * appearance_count
    })
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace().take(2);
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        right.push(iter.next().unwrap().parse::<i32>().unwrap());
    }
    (left, right)
}
