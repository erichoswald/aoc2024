use std::ops::IndexMut;
use itertools::Itertools;

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
    let sequence_offers = build_sequence_offers(initial_secrets);
    let mut max_banana_count = 0;
    let mut sequence = Sequence::min();
    while !sequence.is_max() {
        let sequence_index = sequence.index();
        let mut banana_count = 0;
        for buyer in 0..initial_secrets.len() {
            let remaining_buyers = initial_secrets.len() - buyer;
            if banana_count + remaining_buyers * 9 < max_banana_count {
                break; // It is impossible to surpass the current maximum.
            }
            if let Some(offer) = sequence_offers[buyer][sequence_index] {
                banana_count += offer as usize;
            }
        }
        max_banana_count = max_banana_count.max(banana_count);
        sequence.increment();
    }
    println!("Part 2: {}", max_banana_count);
}

fn build_sequence_offers(initial_secrets: &Vec<u64>) -> Vec<Vec<Option<u8>>> {
    let mut buyer_offers = Vec::new();
    for initial_secret in initial_secrets {
        let mut offers = vec![None; SEQUENCES];
        let mut secret = *initial_secret;
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
            offers.index_mut(sequence.index()).get_or_insert(offer as u8);
        }
        buyer_offers.push(offers);
    }
    buyer_offers
}

const SEQUENCES: usize = 19 * 19 * 19 * 19;

#[derive(Debug)]
struct Sequence {
    changes: [i8; 4],
}

impl Sequence {
    fn min() -> Sequence {
        Sequence { changes: [-9; 4] }
    }

    fn from_index(index: usize) -> Sequence {
        let mut changes = [0; 4];
        Self::index_into(index, &mut changes);
        Sequence { changes }
    }

    fn index_into(index: usize, changes: &mut [i8; 4]) {
        changes[0] = (index / (19 * 19 * 19)) as i8 - 9;
        changes[1] = (index % (19 * 19 * 19) / (19 * 19)) as i8 - 9;
        changes[2] = (index % (19 * 19) / 19) as i8 - 9;
        changes[3] = (index % 19) as i8 - 9;
    }

    fn shift(&mut self, change: i8) {
        self.changes[0] = self.changes[1];
        self.changes[1] = self.changes[2];
        self.changes[2] = self.changes[3];
        self.changes[3] = change;
    }

    fn increment(&mut self) {
        Self::index_into(self.index() + 1, &mut self.changes);
    }

    fn is_max(&self) -> bool {
        self.changes.iter().all_equal_value() == Ok(&9)
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
