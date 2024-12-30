use crate::Op::{AND, OR, XOR};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Sample 1");
    let sample = include_str!("../../inputs/sample24_1.txt");
    let (signals, gates) = parse_input(sample);
    part1(&signals, &gates);

    println!("\nSample 2");
    let sample = include_str!("../../inputs/sample24_2.txt");
    let (signals, gates) = parse_input(sample);
    part1(&signals, &gates);

    println!("\nPuzzle");
    let input = include_str!("../../inputs/day24.txt");
    let (signals, gates) = parse_input(input);
    part1(&signals, &gates);
    part2(&gates);
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    AND, OR, XOR,
}

impl Op {
    fn evaluate(&self, x: u64, y: u64) -> u64 {
        match self {
            AND => x & y,
            OR => x | y,
            XOR => x ^ y,
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
                    "AND" => AND,
                    "OR" => OR,
                    "XOR" => XOR,
                    _ => unreachable!(),
                },
                x, y
            };
            (output, gate)
        })
        .collect();
    (signals, gates)
}

fn part1(signals: &HashMap<&str, u64>, gates: &HashMap<&str, Gate>) {
    let mut z_names = gates.keys().filter(|name| name.starts_with("z")).collect::<Vec<_>>();
    z_names.sort();
    let outputs = evaluate_outputs(&signals, &gates, &z_names);
    let output_number = z_names.iter().zip(outputs).fold(0u64, |acc, (name, output)| {
        let bit = &name[1..].parse::<u64>().unwrap();
        acc + output * (1u64 << bit)
    });
    println!("Part 1: {output_number}");
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

fn part2(gates: &HashMap<&str, Gate>) {
    /*
      For each bit except bit 0, the adder contains two half-adders:
       * A gate computing x[bit] XOR y[bit] (input_sum[bit])
       * A gate computing x[bit] AND y[bit] (input_carry[bit])
       * A gate computing z[bit] = input_sum[bit] XOR carry_overflow[bit - 1] (output_sum[bit])
       * A gate computing input_sum[bit] AND carry_overflow[bit - 1] (output_carry[bit])
       * A gate computing the carry overflow to the next bit input_carry[bit] OR output_carry[bit]
      Starting with the overflow carry of bit 0, we traverse the gate network as long as we can
      find matching gates or as long as we can swap gates. When we run out of nodes, we backtrack.
     */

    let mut gates: HashMap<&str, &Gate> = gates.iter().map(|(key, value)| (*key, value)).collect();

    // Assume the gates for bit 0 are correct and make them unavailable for matching.
    let z00 = find_gate(&gates, XOR, "x00").unwrap().0;
    assert_eq!(z00, "z00");
    gates.remove(z00); // x00 XOR y00 -> z00
    let overflow = find_gate(&gates, AND, "x00").unwrap(); // x00 AND y00
    gates.remove(overflow.0);
    check_output_sum(1, &overflow.0, &gates, &HashMap::new());
}

fn check_output_sum<'a>(bit: usize, overflow: &str, unused_gates: &HashMap<&'a str, &'a Gate>, swapped_gates: &HashMap<&'a str, &'a str>) {
    // The expected gate must XOR the given carry overflow with the result of XORing the matching inputs.
    if let Some((output_name, input_sum)) = find_gate(unused_gates, XOR, overflow) {
        let unused_gates = remove_gate(unused_gates, output_name);
        let z = format!("z{:02}", bit);
        if output_name == z {
            // Only continue if the current output pin matches the expected bit.
            check_output_carry(bit, input_sum, overflow, &unused_gates, swapped_gates);
        } else if can_swap(swapped_gates, output_name, z.as_str()) {
            // Some other gate produces the expected output, swap with that one.
            if let Some(swapped_gate) = unused_gates.get(z.as_str()) {
                let unused_gates = swap_gate(&unused_gates, z.as_str(), swapped_gate, output_name);
                let swapped_gates = include_swapped_names(swapped_gates, output_name, z.as_str());
                check_output_carry(bit, input_sum, overflow, &unused_gates, &swapped_gates);
            }
        }
    }
}

fn check_output_carry<'a>(bit: usize, input_sum: &str, overflow: &str, unused_gates: &HashMap<&'a str, &'a Gate>, swapped_gates: &HashMap<&'a str, &'a str>) {
    // The expected gate must AND the given carry overflow and input sum.
    if let Some(output_carry) = find_gate_with_inputs(unused_gates, AND, overflow, input_sum) {
        let unused_gates = remove_gate(unused_gates, output_carry);
        check_input_sum(bit, input_sum, output_carry, &unused_gates, swapped_gates);
    }
}

fn check_input_sum<'a>(bit: usize, input_sum: &str, output_carry: &str, unused_gates: &HashMap<&'a str, &'a Gate>, swapped_gates: &HashMap<&'a str, &'a str>) {
    let x = format!("x{:02}", bit);
    if let Some((gate_name, _)) = find_gate(&unused_gates, XOR, &x) {
        let unused_gates = remove_gate(unused_gates, gate_name);
        if gate_name == input_sum {
            check_input_carry(bit, output_carry, &unused_gates, swapped_gates);
        } else if let Some(swapped_gate) = unused_gates.get(input_sum) {
            if can_swap(swapped_gates, input_sum, gate_name) {
                let unused_gates = swap_gate(&unused_gates, input_sum, swapped_gate, gate_name);
                let swapped_gates = include_swapped_names(swapped_gates, gate_name, input_sum);
                check_input_carry(bit, output_carry, &unused_gates, &swapped_gates);
            }
        }
    }
}

fn check_input_carry<'a>(bit: usize, output_carry: &str, unused_gates: &HashMap<&'a str, &'a Gate>, swapped_gates: &HashMap<&'a str, &'a str>) {
    let x = format!("x{:02}", bit);
    if let Some((input_carry, _)) = find_gate(&unused_gates, AND, &x) {
        let unused_gates = remove_gate(unused_gates, input_carry);
        check_overflow(bit, input_carry, output_carry, &unused_gates, swapped_gates);

        // We know nothing about the gate except that it must be an XOR gate consuming x/y inputs.
        for (swapped_name, swapped_gate) in unused_gates.iter().filter(|(_, gate)| gate.is_input_carry()) {
            if can_swap(swapped_gates, input_carry, swapped_name) {
                let unused_gates = swap_gate(&unused_gates, swapped_name, swapped_gate, input_carry);
                let swapped_gates = include_swapped_names(swapped_gates, input_carry, swapped_name);
                check_overflow(bit, swapped_name, output_carry, &unused_gates, &swapped_gates);
            }
        }
    }
}

fn check_overflow<'a>(bit: usize, input_carry: &str, output_carry: &str, unused_gates: &HashMap<&'a str, &'a Gate>, swapped_gates: &HashMap<&'a str, &'a str>) {
    if let Some(overflow) = find_gate_with_inputs(&unused_gates, OR, input_carry, output_carry) {
        let unused_gates = remove_gate(unused_gates, overflow);
        if !unused_gates.is_empty() {
            check_output_sum(bit + 1, overflow, &unused_gates, swapped_gates);
        } else if swapped_gates.len() == 8 {
            let mut wires = swapped_gates.keys().into_iter().map(|n| *n).collect::<Vec<_>>();
            wires.sort();
            println!("Part 2: {}", wires.join(","));
        }
    }
}

fn remove_gate<'a>(gates: &HashMap<&'a str, &'a Gate>, name: &str) -> HashMap<&'a str, &'a Gate<'a>> {
    let mut gates = gates.clone();
    gates.remove(name);
    gates
}

fn swap_gate<'a>(gates: &HashMap<&'a str, &'a Gate>, swapped_name: &'a str, swapped_gate: &'a Gate, original_name: &'a str) -> HashMap<&'a str, &'a Gate<'a>> {
    let mut gates = gates.clone();
    gates.remove(swapped_name);
    gates.insert(original_name, swapped_gate);
    gates
}

fn can_swap(swapped_gates: &HashMap<&str, &str>, a: &str, b: &str) -> bool {
    swapped_gates.len() < 8 && !swapped_gates.contains_key(a) && !swapped_gates.contains_key(b)
}

fn include_swapped_names<'a>(swapped_gates: &HashMap<&'a str, &'a str>, a: &'a str, b: &'a str) -> HashMap<&'a str, &'a str> {
    let mut swapped_gates = swapped_gates.clone();
    swapped_gates.insert(a, b);
    swapped_gates.insert(b, a);
    swapped_gates
}

fn find_gate<'a>(gates: &HashMap<&'a str, &Gate<'a>>, op: Op, op_name: &'a str) -> Option<(&'a str, &'a str)> {
    gates.iter()
        .find(|(_, gate)| gate.op == op && (gate.x == op_name || gate.y == op_name))
        .map(|(name, gate)| (*name, if gate.x == op_name { gate.y } else { gate.x }))
}

fn find_gate_with_inputs<'a>(gates: &HashMap<&'a str, &Gate<'a>>, op: Op, x: &'a str, y: &'a str) -> Option<&'a str> {
    gates.iter()
        .find(|(_, gate)| gate.op == op && (gate.x == x && gate.y == y || gate.y == x && gate.x == y))
        .map(|(name, _)| *name)
}

impl Gate<'_> {
    fn is_input_carry(&self) -> bool {
        self.op == AND && self.is_input_consumer()
    }

    fn is_input_consumer(&self) -> bool {
        (self.x.starts_with("x") || self.x.starts_with("y")) && (self.y.starts_with("x") || self.y.starts_with("y"))
    }
}
