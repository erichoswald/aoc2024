use regex::Regex;

fn main() {
    let sample01 = include_str!("../../inputs/sample03_01.txt");
    let sample02 = include_str!("../../inputs/sample03_02.txt");
    println!("Part 1 (sample): {}", part1(&sample01));
    println!("Part 2 (sample): {}\n", part2(&sample02));

    let input = include_str!("../../inputs/day03.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input.lines().fold(0, |acc, line| {
        re.captures_iter(line).fold(acc, |acc, caps| {
            acc + mul(&caps)
        })
    })
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let (acc, _) = input.lines().fold((0, true), |(acc, enabled), line| {
        re.captures_iter(line).fold((acc, enabled), |(acc, enabled), caps| {
            match caps.get(0).unwrap().as_str() {
                "do()" => (acc, true),
                "don't()" => (acc, false),
                _ => if enabled { (acc + mul(&caps), enabled) } else { (acc, enabled) }
            }
        })
    });
    acc
}

fn mul(caps: &regex::Captures) -> i32 {
    let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    x * y
}
