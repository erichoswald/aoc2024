use std::collections::HashMap;

fn main() {
    let sample = include_str!("../../inputs/sample11.txt");
    println!("Part 1 (sample): {}", sum_resulting_stone_counts(&sample, 25));
    println!("Part 2 (sample): {}\n", sum_resulting_stone_counts(&sample, 75));

    let input = include_str!("../../inputs/day11.txt");
    println!("Part 1: {}", sum_resulting_stone_counts(&input, 25));
    println!("Part 2: {}", sum_resulting_stone_counts(&input, 75));
}

fn sum_resulting_stone_counts(input: &str, blink_count: u32) -> usize {
    let mut stone_counts: HashMap<(u64, u32), usize> = HashMap::new();
    parse_input(input)
        .iter()
        .map(|stone| count_resulting_stones(*stone, blink_count, &mut stone_counts))
        .sum()
}

fn count_resulting_stones(stone: u64, blink_count: u32, stone_counts: &mut HashMap<(u64, u32), usize>) -> usize {
    let cache_key = (stone, blink_count);
    match stone_counts.get(&cache_key) {
        Some(result) => *result,
        None => {
            let computed =
                if blink_count == 0 {
                    1
                } else if stone == 0 {
                    count_resulting_stones(1, blink_count - 1, stone_counts)
                } else {
                    let digits = stone.to_string();
                    let digit_count = digits.len();
                    if digit_count % 2 == 0 {
                        let split = digit_count / 2;
                        let left = *(&digits[..split].parse::<u64>().unwrap());
                        let right = *(&digits[split..].parse::<u64>().unwrap());
                        count_resulting_stones(left, blink_count - 1, stone_counts)
                            + count_resulting_stones(right, blink_count - 1, stone_counts)
                    } else {
                        count_resulting_stones(stone * 2024, blink_count - 1, stone_counts)
                    }
                };
            stone_counts.insert((stone, blink_count), computed);
            computed
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
