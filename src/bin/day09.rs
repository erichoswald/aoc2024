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
    let mut blocks = parse_disk_map(input);
    compact(&mut blocks[..]);
    checksum(&blocks)
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

fn compact(blocks: &mut [i32]) {
    let mut segments = Vec::new();
    let mut start = 0usize;
    let mut end = start;
    let mut id = blocks.first().unwrap();
    for (index, b) in blocks.iter().enumerate() {
        if b == id {
            if start < end {
                end += 1;
            } else {
                start = index;
                end = index + 1;
            }
        } else if start < end {
            segments.push((*id, start, end));
            id = b;
            start = index;
            end = start + 1;
        }
    }
    if start < end {
        segments.push((*id, start, end));
    }
    segments.push((-1, end, end + 1)); // Sentinel

    let mut id = *id;
    while id >= 0 {
        let mut source_index = 0;
        while (segments[source_index].0 != id) {
            source_index += 1;
        }
        let source = segments[source_index].clone();
        let len = source.2 - source.1;
        let mut target_index = 0;
        while (target_index < segments.len() && (segments[target_index].0 >= 0 || segments[target_index].1 + len > segments[target_index].2)) {
            target_index += 1;
        }
        if (target_index < segments.len() && segments[target_index].1 < source.1) {
            let target = segments[target_index].clone();
            segments[source_index] = (-1, source.1, source.2);
            let end = target.1 + len;
            let rest = target.2 - end;
            segments[target_index] = (source.0, target.1, end);
            if (rest > 0) {
                let next_index = target_index + 1;
                if segments[next_index].0 >= 0 {
                    segments.insert(next_index, (-1, end, end + rest))
                } else {
                    segments[next_index] = (-1, end, segments[next_index].2)
                }
            }
        }
        id -= 1;
    }

    for segment in segments {
        for i in segment.1..segment.2 {
            if (i < blocks.len()) {
                blocks[i] = segment.0
            }
        }
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
