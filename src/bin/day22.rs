fn main() {
    let sample = include_str!("../../inputs/sample22.txt");
    println!("Sample");
    solve(&sample);

    let input = include_str!("../../inputs/day22.txt");
    println!("\nPuzzle");
    solve(&input);
}

fn solve(input: &str) {
    let sum = input.lines()
        .map(|line| { generate(2000, line.parse::<u64>().unwrap()) })
        .sum::<u64>();
    println!("Part 1: {}", sum);
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
