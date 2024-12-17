fn main() {
    let sample = include_str!("../../inputs/sample17.txt");
    let input = include_str!("../../inputs/day17.txt");
    println!("Sample outputs: {}", format_outputs(&solve(&sample)));
    println!("Puzzle outputs: {}", format_outputs(&solve(&input)));
}

#[derive(Debug)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

fn solve(input: &str) -> Vec<u32> {
    let (mut registers, program) = parse_program(input);
    let mut ip = 0;
    let mut outputs = Vec::new();
    loop {
        let operand = program.get(ip + 1);
        match program.get(ip) {
            Some(0) => {
                registers.a = registers.a / (1 << combo(operand, &registers));
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
                registers.b = registers.a / (1 << combo(operand, &registers));
                ip += 2
            }
            Some(7) => {
                registers.c = registers.a / (1 << combo(operand, &registers));
                ip += 2
            }
            None => break,
            _ => panic!("Invalid instruction {ip}")
        }
    }
    outputs
}

fn literal(operand: Option<&u8>) -> u32 {
    *operand.unwrap() as u32
}

fn combo(operand: Option<&u8>, registers: &Registers) -> u32 {
    let operand = *operand.unwrap();
    match operand {
        0..=3 => operand as u32,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Invalid combo operand {operand}")
    }
}

fn format_outputs(outputs: &[u32]) -> String {
    outputs.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(",")
}

fn parse_program(input: &str) -> (Registers, Vec<u8>) {
    let lines = input.lines().collect::<Vec<_>>();
    let registers = Registers {
        a: lines[0].strip_prefix("Register A: ").unwrap().parse().unwrap(),
        b: lines[1].strip_prefix("Register B: ").unwrap().parse().unwrap(),
        c: lines[2].strip_prefix("Register C: ").unwrap().parse().unwrap(),
    };
    let program = lines[4].strip_prefix("Program: ").unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    (registers, program)
}
