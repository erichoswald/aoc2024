fn main() {
    let sample = include_str!("../../inputs/sample09.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day09.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut blocks = parse_disk_map(input);
    defragment(&mut blocks[..]);
    checksum(&blocks)
}

fn part2(input: &str) -> usize {
    0
}

fn defragment(blocks: &mut [i32]) {
    let mut head = 0;
    let mut tail = blocks.len() - 1;
    while head < tail {
        while blocks[tail] < 0 {
            tail -= 1;
        }
        if head < tail && blocks[head] < 0 {
            blocks.swap(head, tail);
            tail -= 1;
        }
        head += 1;
    }
}

fn checksum(blocks: &[i32]) -> usize {
    let mut sum = 0;
    for (index, id) in blocks.iter().enumerate() {
        if (*id >= 0) {
            sum += index * id.abs() as usize;
        }
    }
    sum
}

fn parse_disk_map(input: &str) -> Vec<i32> {
    let mut blocks = Vec::new();
    let first_line = input.lines().next().unwrap();
    let mut bytes: Vec<i32> = first_line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let mut id = 0;
    let mut i = 0;
    while i < bytes.len() {
        let file_block_count = bytes[i];
        for _ in 0..file_block_count {
            blocks.push(id);
        }
        i += 1;
        if i < bytes.len() {
            let free_block_count = bytes[i];
            for _ in 0..free_block_count {
                blocks.push(-1);
            }
            i += 1;
        }
        id += 1;
    }
    blocks
}
