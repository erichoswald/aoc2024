fn main() {
    let sample = include_str!("../../inputs/sample02.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day02.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let reports = parse_input(input);
    reports
        .iter()
        .map(|report| is_safe(report))
        .filter(|report| *report)
        .count()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut sign = 0;
    let deltas = report.iter().zip(report.iter().skip(1)).map(|(a, b)| b - a);
    for delta in deltas {
        if sign * delta < 0 || delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }
        sign = delta
    }
    true
}

fn part2(input: &str) -> i32 {
    0
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
