fn main() {
    let sample = include_str!("../../inputs/sample17.txt");
    let input = include_str!("../../inputs/day17.txt");
    let (mut sample_registers, sample_program) = parse_program(&sample);
    let mut sample_outputs = Vec::new();
    solve(&sample_program, &mut sample_registers, &mut sample_outputs);
    println!("Sample outputs: {}", format_outputs(&sample_outputs));
    let (mut registers, program) = parse_program(&input);
    let mut outputs = Vec::new();
    solve(&program, &mut registers, &mut outputs);
    println!("Puzzle outputs: {}", format_outputs(&outputs));

    let a = reproduce(&program, 0, program.len() - 1).unwrap();
    let mut registers = Registers { a, b: 0, c: 0 };
    let mut outputs = Vec::new();
    solve(&program, &mut registers, &mut outputs);
    println!("Part 2: a={a} outputs={}", format_outputs(&outputs));
}

#[derive(Debug, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn reproduce(program: &Vec<u64>, a: u64, from_index: usize) -> Option<u64> {
    // At least in my input the sequence ends with an `adv 3` (cutting off the last 3 bits)
    // and a `jnz 0` that goes back to the start until a becomes 0.
    // Hence, the first 3 digits of a drive the last output, the second 3 digits the second to last
    // output, and so on.
    // The algorithm reproduces the program in reverse order, adding the lowest 3 digits that
    // produce the next desired output and that don't lead into a dead end.
    for bits in 0..8 {
        let a = (a << 3) + bits;
        let mut registers = Registers { a, b: 0, c: 0 };
        let mut outputs = Vec::new();
        solve(&program, &mut registers, &mut outputs);
        if outputs.as_slice() == &program[from_index..] {
            println!("Match from {from_index} with {a}");
            if from_index == 0 {
                return Some(a)
            } else {
                let r = reproduce(program, a, from_index - 1);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    None
}

fn solve(program: &Vec<u64>, registers: &mut Registers, outputs: &mut Vec<u64>) {
    let mut ip = 0;
    loop {
        let operand = program.get(ip + 1);
        match program.get(ip) {
            Some(0) => {
                registers.a = registers.a >> combo(operand, &registers);
                ip += 2
            },
            Some(1) => {
                registers.b = registers.b ^ literal(operand);
                ip += 2
            }
            Some(2) => {
                registers.b = combo(operand, &registers) % 8;
                ip += 2
            }
            Some(3) => {
                if registers.a == 0 {
                    ip += 2
                } else {
                    ip = *operand.unwrap() as usize;
                }
            }
            Some(4) => {
                registers.b = registers.b ^ registers.c;
                ip += 2
            }
            Some(5) => {
                outputs.push(combo(operand, &registers) % 8);
                ip += 2
            }
            Some(6) => {
                registers.b = registers.a >> combo(operand, &registers);
                ip += 2
            }
            Some(7) => {
                registers.c = registers.a >> combo(operand, &registers);
                ip += 2
            }
            None => break,
            _ => panic!("Invalid instruction {ip}")
        }
    }
}

fn literal(operand: Option<&u64>) -> u64 {
    *operand.unwrap()
}

fn combo(operand: Option<&u64>, registers: &Registers) -> u64 {
    let operand = *operand.unwrap();
    match operand {
        0..=3 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Invalid combo operand {operand}")
    }
}

fn format_outputs(outputs: &[u64]) -> String {
    outputs.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(",")
}

fn parse_program(input: &str) -> (Registers, Vec<u64>) {
    let lines = input.lines().collect::<Vec<_>>();
    let registers = Registers {
        a: lines[0].strip_prefix("Register A: ").unwrap().parse().unwrap(),
        b: lines[1].strip_prefix("Register B: ").unwrap().parse().unwrap(),
        c: lines[2].strip_prefix("Register C: ").unwrap().parse().unwrap(),
    };
    let program = lines[4].strip_prefix("Program: ").unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (registers, program)
}
