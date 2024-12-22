use std::collections::HashMap;

fn main() {
    let sample = include_str!("../../inputs/sample21.txt");
    println!("Sample");
    solve(&sample);

    let input = include_str!("../../inputs/day21.txt");
    println!("\nPuzzle");
    solve(&input);
}

struct Keypad {
    moves: HashMap<(char, char), Vec<String>>,
}

impl Keypad {
    fn numeric() -> Self {
        let mut keypad = Keypad { moves: HashMap::new() };
        keypad.add_keys("A0123456789");
        keypad.add_sequences('A', '0', &vec!["<"]);
        keypad.add_sequences('A', '1', &vec!["^<<", "<^<"]);
        keypad.add_sequences('A', '2', &vec!["^<", "<^"]);
        keypad.add_sequences('A', '3', &vec!["^"]);
        keypad.add_sequences('A', '4', &vec!["^^<<", "^<<^", "<^^<", "<^<^"]);
        keypad.add_sequences('A', '5', &vec!["^^<", "^<^", "<^^"]);
        keypad.add_sequences('A', '6', &vec!["^^"]);
        keypad.add_sequences('A', '7', &vec!["^^^<<", "^^<^<", "^^<<^", "^<^^<", "^<<^^", "<^^^<", "<^<^^"]);
        keypad.add_sequences('A', '8', &vec!["^^^<", "^^<^", "^<^^", "<^^^"]);
        keypad.add_sequences('A', '9', &vec!["^^^"]);
        keypad.add_sequences('0', '1', &vec!["^<"]);
        keypad.add_sequences('0', '2', &vec!["^"]);
        keypad.add_sequences('0', '3', &vec!["^>", ">^"]);
        keypad.add_sequences('0', '4', &vec!["^^<", "^<^"]);
        keypad.add_sequences('0', '5', &vec!["^^"]);
        keypad.add_sequences('0', '6', &vec!["^^>", ">^^"]);
        keypad.add_sequences('0', '7', &vec!["^^^<", "^^<^", "^<^^"]);
        keypad.add_sequences('0', '8', &vec!["^^^"]);
        keypad.add_sequences('0', '9', &vec!["^^^>", "^^>^", "^>^^", ">^^^"]);
        keypad.add_sequences('1', '2', &vec![">"]);
        keypad.add_sequences('1', '3', &vec![">>"]);
        keypad.add_sequences('1', '4', &vec!["^"]);
        keypad.add_sequences('1', '5', &vec!["^>", ">^"]);
        keypad.add_sequences('1', '6', &vec!["^>>", ">^>", ">>^"]);
        keypad.add_sequences('1', '7', &vec!["^^"]);
        keypad.add_sequences('1', '7', &vec!["^^"]);
        keypad.add_sequences('1', '8', &vec!["^^>", "^>^", ">^^"]);
        keypad.add_sequences('1', '9', &vec!["^^>>", "^>^>", "^>>^", ">^^>", ">^>^", ">>^^"]);
        keypad.add_sequences('2', '3', &vec![">"]);
        keypad.add_sequences('2', '4', &vec!["^<", "<^"]);
        keypad.add_sequences('2', '5', &vec!["^"]);
        keypad.add_sequences('2', '6', &vec!["^>", ">^"]);
        keypad.add_sequences('2', '7', &vec!["^^<", "^<^", "<^^"]);
        keypad.add_sequences('2', '8', &vec!["^^"]);
        keypad.add_sequences('2', '9', &vec!["^^>", "^>^", ">^^"]);
        keypad.add_sequences('3', '4', &vec!["^<<", "^<^", "<<^"]);
        keypad.add_sequences('3', '5', &vec!["^<", "<^"]);
        keypad.add_sequences('3', '6', &vec!["^"]);
        keypad.add_sequences('3', '7', &vec!["^^<<", "^<^<", "^<<^", "<^^<", "<^<^", "<<^^"]);
        keypad.add_sequences('3', '8', &vec!["^^<", "^<^", "<^^"]);
        keypad.add_sequences('3', '9', &vec!["^^"]);
        keypad.add_sequences('4', '5', &vec![">"]);
        keypad.add_sequences('4', '6', &vec![">>"]);
        keypad.add_sequences('4', '7', &vec!["^"]);
        keypad.add_sequences('4', '8', &vec!["^>", ">^"]);
        keypad.add_sequences('4', '9', &vec!["^>>", ">^>", ">>^"]);
        keypad.add_sequences('5', '6', &vec![">"]);
        keypad.add_sequences('5', '7', &vec!["^<", "<^"]);
        keypad.add_sequences('5', '8', &vec!["^"]);
        keypad.add_sequences('5', '9', &vec!["^>", ">^"]);
        keypad.add_sequences('6', '7', &vec!["^<<", "<^<", "<<^"]);
        keypad.add_sequences('6', '8', &vec!["^<", "<^"]);
        keypad.add_sequences('6', '9', &vec!["^"]);
        keypad.add_sequences('7', '9', &vec![">>"]);
        keypad.add_sequences('7', '8', &vec![">"]);
        keypad.add_sequences('8', '9', &vec![">"]);
        keypad
    }

    fn directional() -> Self {
        let mut keypad = Keypad { moves: HashMap::new() };
        keypad.add_keys("A^v<>");
        keypad.add_sequences('A', '^', &vec!["<"]);
        keypad.add_sequences('A', 'v', &vec!["<v", "v<"]);
        keypad.add_sequences('A', '<', &vec!["v<<", "<v<"]);
        keypad.add_sequences('A', '>', &vec!["v"]);
        keypad.add_sequences('^', 'v', &vec!["v"]);
        keypad.add_sequences('^', '<', &vec!["v<"]);
        keypad.add_sequences('^', '>', &vec!["v>", ">v"]);
        keypad.add_sequences('v', '<', &vec!["<"]);
        keypad.add_sequences('v', '>', &vec![">"]);
        keypad.add_sequences('<', '>', &vec![">>"]);
        keypad
    }

    fn add_keys(&mut self, keys: &str) {
        for key in keys.chars() {
            self.add_sequences(key, key, &vec![""]);
        }
    }

    fn add_sequences(&mut self, from: char, to: char, sequences: &Vec<&str>) {
        let mut reverse_sequences = Vec::new();
        for sequence in sequences {
            reverse_sequences.push(sequence.chars().rev().map(|c| invert_move(c)).collect::<String>());
        }
        self.moves.insert((to, from), reverse_sequences);
        self.moves.insert((from, to), sequences.iter().map(|s| s.to_string()).collect());
    }

    fn find_sequences(&self, from: char, keys: &str, path: &str, sequences: &mut Vec<String>) {
        if let Some(to) = keys.chars().next() {
            let moves = self.moves.get(&(from, to)).unwrap();
            for m in moves {
                let mut new_path = String::from(path);
                new_path.push_str(m);
                new_path.push('A');
                self.find_sequences(to, &keys[1..], &new_path, sequences);
            }
        } else {
            sequences.push(String::from(path));
        }
    }
}

fn invert_move(key: char) -> char {
    match key {
        '^' => 'v',
        'v' => '^',
        '<' => '>',
        '>' => '<',
        _ => panic!("Unknown movement: {key}")
    }
}

fn solve(input: &str) {
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();
    let mut total_complexity = 0;
    for line in input.lines() {
        let mut s1 = Vec::new();
        let mut sequences = Vec::new();
        numeric.find_sequences('A', line, "", &mut sequences);
        for sequence in sequences {
            let mut s0 = Vec::new();
            directional.find_sequences('A', sequence.as_str(), "", &mut s0);
            for ss in s0 {
                directional.find_sequences('A', ss.as_str(), "", &mut s1);
            }
        }
        let length = s1.iter().map(|s| s.len()).min().unwrap();
        total_complexity += length * numeric_part(line);
    }
    println!("{}", total_complexity);
}

fn numeric_part(line: &str) -> usize {
    line.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<usize>().unwrap()
}
