fn main() {
    let sample = include_str!("../../inputs/sample07.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day07.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let equations = parse_input(input);
    let mut total_calibration = 0;
    for equation in equations {
        if is_solvable(equation.0, &equation.1, 0) {
            total_calibration += equation.0;
        }
    }
    total_calibration
}

fn part2(input: &str) -> u64 {
    0
}

fn is_solvable(test_value: u64, operands: &[u64], accumulated: u64) -> bool {
    if accumulated > test_value {
        false
    } else {
        match operands {
            [operand, ..] => {
                is_solvable(test_value, &operands[1..], accumulated + operand)
                || is_solvable(test_value, &operands[1..], accumulated * operand)
            }
            [] => accumulated == test_value
        }
    }
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
