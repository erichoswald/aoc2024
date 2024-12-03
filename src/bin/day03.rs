use regex::Regex;

fn main() {
    let sample = include_str!("../../inputs/sample03.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day03.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input.lines().fold(0, |acc, line| {
        re.captures_iter(line).fold(acc, |a, caps| {
            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            a + x * y
        })
    })
}

fn part2(input: &str) -> usize {
    0
}
