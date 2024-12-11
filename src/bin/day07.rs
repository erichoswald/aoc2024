use std::ops::Add;

fn main() {
    let sample = include_str!("../../inputs/sample07.txt");
    println!("Part 1 (sample): {}", sum_calibrations(&sample, &[add, multiply]));
    println!("Part 2 (sample): {}\n", sum_calibrations(&sample, &[add, multiply, concatenate]));

    let input = include_str!("../../inputs/day07.txt");
    println!("Part 1: {}", sum_calibrations(&input, &[add, multiply]));
    println!("Part 2: {}", sum_calibrations(&input, &[add, multiply, concatenate]));
}

fn sum_calibrations(input: &str, operators: &[fn(u64, u64) -> u64]) -> u64 {
    let equations = parse_input(input);
    let mut total_calibration = 0;
    for equation in equations {
        if is_solvable(equation.0, &equation.1, operators, 0) {
            total_calibration += equation.0;
        }
    }
    total_calibration
}

fn is_solvable(
    test_value: u64,
    operands: &[u64],
    operators: &[fn(u64, u64) -> u64],
    accumulated: u64,
) -> bool {
    if accumulated > test_value {
        false
    } else {
        match operands {
            [operand, ..] => {
                operators
                    .iter()
                    .fold(false, |is_solved, operation| {
                        is_solved || is_solvable(test_value, &operands[1..], operators, operation(accumulated, *operand))
                    })
            }
            [] => accumulated == test_value
        }
    }
}

fn add(x: u64, y: u64) -> u64 {
    x + y
}

fn multiply(x: u64, y: u64) -> u64 {
    x * y
}

fn concatenate(x: u64, y: u64) -> u64 {
    x.to_string().add(y.to_string().as_str()).parse::<u64>().unwrap()
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(": ");
            let test_value = line_iter.next().unwrap().parse::<u64>().unwrap();
            let operands = line_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|word| word.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (test_value, operands)
        })
        .collect()
}
