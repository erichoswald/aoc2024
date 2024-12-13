use itertools::Itertools;
use regex::Regex;

fn main() {
    let sample = include_str!("../../inputs/sample13.txt");
    println!("Part 1 (sample): {}", count_total_tokens(&sample, 0));
    println!("Part 2 (sample): {}\n", count_total_tokens(&sample, 10000000000000));

    let input = include_str!("../../inputs/day13.txt");
    println!("Part 1: {}", count_total_tokens(&input, 0));
    println!("Part 2: {}", count_total_tokens(&input, 10000000000000));
}

fn count_total_tokens(input: &str, offset: i64) -> i64 {
    let machines = parse_machines(&input);
    let mut total_tokens = 0;
    for machine in &machines {
        if let Some(tokens) = machine.winning_tokens(offset) {
            total_tokens += tokens;
        }
    }
    total_tokens
}

impl Machine {
    fn winning_tokens(&self, offset: i64) -> Option<i64> {
        // a * ax + b * bx - px = 0; b = (px - a * ax) / bx
        // a * ay + b * by - py = 0
        // a * ay + (px - a * ax) / bx * by - py = 0
        // a * ay * bx + px * by - a * ax * by - py * bx = 0
        // a * (ay * bx - ax * by) = py * bx - px * by
        // a = (py * bx - px * by) / (ay * bx - ax * by)
        let px = self.px + offset;
        let py = self.py + offset;
        let nom = py * self.bx - px * self.by;
        let denom = self.ay * self.bx - self.ax * self.by;
        let a = Self::divide(nom, denom)?;
        let b = Self::divide(px - a * self.ax, self.bx)?;
        Some(a * 3 + b)
    }

    fn divide(nom: i64, denom: i64) -> Option<i64> {
        let d = nom / denom;
        if d * denom == nom { Some(d) } else { None }
    }
}

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let button_a_pattern = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_pattern = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut machines: Vec<Machine> = Vec::new();
    for lines in input.lines().chunks(4).into_iter() {
        let lines: Vec<&str> = lines.into_iter().collect();
        let button_a_captures = button_a_pattern.captures(lines[0]).unwrap();
        let button_b_captures = button_b_pattern.captures(lines[1]).unwrap();
        let prize_captures = prize_pattern.captures(lines[2]).unwrap();
        let machine = Machine {
            ax: coordinate_from_capture(&button_a_captures, 1),
            ay: coordinate_from_capture(&button_a_captures, 2),
            bx: coordinate_from_capture(&button_b_captures, 1),
            by: coordinate_from_capture(&button_b_captures, 2),
            px: coordinate_from_capture(&prize_captures, 1),
            py: coordinate_from_capture(&prize_captures, 2),
        };
        machines.push(machine);
    };
    machines
}

fn coordinate_from_capture(capture: &regex::Captures, index: usize) -> i64 {
    capture.get(index).unwrap().as_str().parse::<i64>().unwrap()
}
