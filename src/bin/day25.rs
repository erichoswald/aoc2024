use itertools::Itertools;

fn main() {
    let sample = include_str!("../../inputs/sample25.txt");
    println!("Sample");
    solve(&sample);

    let input = include_str!("../../inputs/day25.txt");
    println!("\nPuzzle");
    solve(&input);
}

fn solve(input: &str) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    parse_locks_and_keys(input, &mut locks, &mut keys);
    let mut fit_count = 0;
    for lock in &locks {
        for key in &keys {
            if is_fit(lock, key) {
                fit_count += 1;
            }
        }
    }
    println!("Number of fitting lock/key pairs: {}", fit_count);
}

fn is_fit(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    for pin in 0..5 {
        if lock[pin] + key[pin] > 5 {
            return false;
        }
    }
    true
}

fn parse_locks_and_keys(input: &str, locks: &mut Vec<Vec<usize>>, keys: &mut Vec<Vec<usize>>) {
    for stack in &input.lines().filter(|line| !line.is_empty()).chunks(7) {
        let lines = stack.collect();
        parse_line_stack(&lines, locks, keys);
    }
}

fn parse_line_stack(lines: &Vec<&str>, locks: &mut Vec<Vec<usize>>, keys: &mut Vec<Vec<usize>>) {
    let mut pins = Vec::with_capacity(5);
    let last = lines.last().unwrap();
    if last.starts_with(".") {
        for col in 0..5 {
            let mut h = 0;
            while lines[h + 1].chars().nth(col).unwrap() == '#' {
                h += 1;
            }
            pins.push(h);
        }
        locks.push(pins);
    } else if last.starts_with("#") {
        for col in 0..5 {
            let mut h = 0;
            while lines[lines.len() - h - 2].chars().nth(col).unwrap() == '#' {
                h += 1;
            }
            pins.push(h);
        }
        keys.push(pins);
    }
}
