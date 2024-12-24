use std::collections::HashMap;
use regex::Regex;

fn main() {
    let sample = include_str!("../../inputs/sample24_1.txt");
    println!("Sample 1");
    solve(&sample);

    let sample = include_str!("../../inputs/sample24_2.txt");
    println!("\nSample 2");
    solve(&sample);

    let input = include_str!("../../inputs/day24.txt");
    println!("\nPuzzle");
    solve(&input);
}

#[derive(Debug)]
enum Op {
    AND, OR, XOR,
}

impl Op {
    fn evaluate(&self, x: u64, y: u64) -> u64 {
        match self {
            Op::AND => x & y,
            Op::OR => x | y,
            Op::XOR => x ^ y,
        }
    }
}

#[derive(Debug)]
struct Gate<'a> {
    op: Op,
    x: &'a str,
    y: &'a str,
}

fn parse_input(input: &str) -> (HashMap<&str, u64>, HashMap<&str, Gate>) {
    let input_regex: Regex = Regex::new(r"([a-z0-9]{3}): (\d)").unwrap();
    let signals: HashMap<&str, u64> = input.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_, [name, signal]) = input_regex.captures(line).unwrap().extract();
            (name, signal.parse::<u64>().unwrap())
        })
        .collect();

    let gate_regex: Regex = Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
    let gates: HashMap<&str, Gate> = input.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let (_, [x, op, y, output]) = gate_regex.captures(line).unwrap().extract();
            let gate = Gate {
                op: match op {
                    "AND" => Op::AND,
                    "OR" => Op::OR,
                    "XOR" => Op::XOR,
                    _ => unreachable!(),
                },
                x, y
            };
            (output, gate)
        })
        .collect();
    (signals, gates)
}

fn solve(input: &str) {
    let (signals, gates) = parse_input(input);
    let mut z_names = gates.keys().filter(|name| name.starts_with("z")).collect::<Vec<_>>();
    z_names.sort();
    let outputs = evaluate_outputs(&signals, &gates, &z_names);
    let output_number = z_names.iter().zip(outputs).fold(0u64, |acc, (name, output)| {
        let bit = &name[1..].parse::<u64>().unwrap();
        acc + output * (1u64 << bit)
    });
    println!("Output: {output_number}");
}

fn evaluate_outputs(signals: &HashMap<&str, u64>, gates: &HashMap<&str, Gate>, outputs: &Vec<&&str>) -> Vec<u64> {
    let mut signals = signals.clone();
    let mut output_signals = Vec::with_capacity(outputs.len());
    for output in outputs {
        output_signals.push(evaluate_output(&mut signals, gates, output));
    }
    output_signals
}

fn evaluate_output<'a>(signals: &mut HashMap<&'a str, u64>, gates: &'a HashMap<&str, Gate>, output: &'a str) -> u64 {
    if let Some(output_signal) = signals.get(output) {
        *output_signal
    } else {
        let gate = gates.get(output).unwrap();
        let x = evaluate_output(signals, gates, gate.x);
        let y = evaluate_output(signals, gates, gate.y);
        let output_signal = gate.op.evaluate(x, y);
        signals.insert(output, output_signal);
        output_signal
    }
}
