use std::collections::HashMap;

fn main() {
    println!("Sample");
    let sample = include_str!("../../inputs/sample22.txt");
    part1(&parse_initial_secrets(sample));

    println!("\nTest part 2");
    part2(&vec![1, 2, 3, 2024]);

    println!("\nPuzzle");
    let input = include_str!("../../inputs/day22.txt");
    let initial_secret = parse_initial_secrets(input);
    part1(&initial_secret);
    part2(&initial_secret);
}

fn parse_initial_secrets(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn part1(initial_secrets: &Vec<u64>) {
    let sum = initial_secrets
        .iter()
        .map(|secret| generate(2000, *secret))
        .sum::<u64>();
    println!("Part 1: {}", sum);
}

fn part2(initial_secrets: &Vec<u64>) {
    let mut offer_sums: HashMap<usize, u64> = HashMap::new();
    for initial_secret in initial_secrets {
        let sequence_offers = build_sequence_offers(*initial_secret);
        for (sequence_index, offer) in sequence_offers.iter() {
            offer_sums.entry(*sequence_index)
                .and_modify(|offer_sum| *offer_sum += *offer as u64)
                .or_insert(*offer as u64);
        }
    }
    let max_banana_count = offer_sums.values().max().unwrap();
    println!("Part 2: {}", max_banana_count);
}

fn build_sequence_offers(initial_secret: u64) -> HashMap<usize, u8> {
    let mut offers = HashMap::new();
    let mut secret = initial_secret;
    let mut previous_offer = (secret % 10) as i8;
    let mut sequence = Sequence::min();
    for _ in 1..4 {
        secret = evolve(secret);
        let offer = (secret % 10) as i8;
        sequence.shift(offer - previous_offer);
        previous_offer = offer;
    }
    for _ in 4..2000 {
        secret = evolve(secret);
        let offer = (secret % 10) as i8;
        sequence.shift(offer - previous_offer);
        previous_offer = offer;

        // Only the first time a specific sequence appears will the current offer be used.
        offers.entry(sequence.index()).or_insert(offer as u8);
    }
    offers
}

#[derive(Debug, PartialEq)]
struct Sequence {
    changes: [i8; 4],
}

impl Sequence {
    fn min() -> Sequence {
        Sequence { changes: [-9; 4] }
    }

    fn shift(&mut self, change: i8) {
        self.changes[0] = self.changes[1];
        self.changes[1] = self.changes[2];
        self.changes[2] = self.changes[3];
        self.changes[3] = change;
    }

    fn index(&self) -> usize {
        (self.changes[3] + 9) as usize
            + (self.changes[2] + 9) as usize * 19
            + (self.changes[1] + 9) as usize * (19 * 19)
            + (self.changes[0] + 9) as usize * (19 * 19 * 19)
    }
}

const PRUNE_MASK: u64 = 0x1000000 - 1;

fn generate(generation_count: usize, initial_secret: u64) -> u64 {
    let mut secret = initial_secret;
    for _ in 0..generation_count {
        secret = evolve(secret);
    }
    secret
}

fn evolve(secret: u64) -> u64 {
    let secret = (secret ^ (secret << 6)) & PRUNE_MASK;
    let secret = (secret ^ secret >> 5) & PRUNE_MASK;
    (secret ^ secret << 11) & PRUNE_MASK
}
