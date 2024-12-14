use std::collections::HashMap;
use regex::Regex;

fn main() {
    let sample = include_str!("../../inputs/sample14.txt");
    println!("Part 1 (sample): {}", part1(&parse_robots(&sample), 11, 7));

    let input = include_str!("../../inputs/day14.txt");
    let robots = parse_robots(&input);
    println!("Part 1: {}", part1(&robots, 101, 103));
    println!("Part 2: {}", part2(&robots, 101, 103));
}

fn part1(robots: &Vec<Robot>, width: i32, height: i32) -> usize {
    let position_counts = count_robot_positions(&robots, width, height, 100);
    safety_factor(&position_counts, width, height)
}

fn part2(robots: &Vec<Robot>, width: i32, height: i32) -> usize {
    0
}

fn count_robot_positions(robots: &Vec<Robot>, width: i32, height: i32, seconds: i32) -> HashMap<(i32, i32), usize> {
    let mut counts = HashMap::new();
    for robot in robots {
        counts.entry(robot.position_within_after(width, height, seconds))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    counts
}

fn safety_factor(position_counts: &HashMap<(i32, i32), usize>, width: i32, height: i32) -> usize {
    let mx = width / 2;
    let my = height / 2;
    count_robots_in_quadrant(position_counts, 0, 0, mx, my)
        * count_robots_in_quadrant(position_counts, mx + 1, 0, width, my)
        * count_robots_in_quadrant(position_counts, 0, my + 1, mx, height)
        * count_robots_in_quadrant(position_counts, mx + 1, my + 1, width, height)
}

fn count_robots_in_quadrant(position_counts: &HashMap<(i32, i32), usize>, x0: i32, y0: i32, x1: i32, y1: i32) -> usize {
    position_counts.iter()
        .filter(|&((x, y), _)| x0 <= *x && *x < x1 && y0 <= *y && *y < y1)
        .map(|(_, count)| count)
        .sum()
}

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn position_within_after(&self, width: i32, height: i32, seconds: i32) -> (i32, i32) {
        let x = (self.px + seconds * self.vx).rem_euclid(width);
        let y = (self.py + seconds * self.vy).rem_euclid(height);
        (x, y)
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let robot_pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = Vec::new();
    for line in input.lines() {
        let captures = robot_pattern.captures(line).unwrap();
        let robot = Robot {
            px: parse_number(&captures, 1),
            py: parse_number(&captures, 2),
            vx: parse_number(&captures, 3),
            vy: parse_number(&captures, 4),
        };
        robots.push(robot);
    }
    robots
}

fn parse_number(captures: &regex::Captures, index: usize) -> i32 {
    captures.get(index).unwrap().as_str().parse::<i32>().unwrap()
}
