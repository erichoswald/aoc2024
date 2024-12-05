use std::collections::{HashMap, HashSet};

fn main() {
    let sample = include_str!("../../inputs/sample05.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    // println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day05.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Input {
    predecessors: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

fn part1(input: &str) -> i32 {
    let input = parse_input(input);
    input.updates.iter().fold(0, |acc, update| {
        if is_update_valid(update.as_slice(), &mut HashSet::new(), &input.predecessors) {
            let middle = update[update.len() / 2];
            acc + middle
        } else {
            acc
        }
    })
}

fn is_update_valid(
    update: &[i32],
    printed_pages: &mut HashSet<i32>,
    predecessors: &HashMap<i32, HashSet<i32>>,
) -> bool {
    match update {
        [page, ..] => {
            for printed_page in printed_pages.iter() {
                if predecessors.get(printed_page).unwrap_or(&HashSet::new()).contains(page) {
                    return false
                }
            }
            printed_pages.insert(*page);
            is_update_valid(&update[1..], printed_pages, predecessors)
        },
        [] => {
            true
        },
    }
}

fn part2(input: &str) -> i32 {
    0
}

fn parse_input(input: &str) -> Input {
    let mut predecessors: HashMap<i32, HashSet<i32>> = HashMap::new();
    input.lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let mut iter = line.split('|').take(2);
            let before = iter.next().unwrap().parse().unwrap();
            let after = iter.next().unwrap().parse().unwrap();
            predecessors.entry(after)
                .and_modify(|set| { set.insert(before); })
                .or_insert(HashSet::from([ before ]));
        });
    let mut updates = Vec::new();
    input.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .for_each(|line| {
            let update = line.split(',').map(|page| page.parse::<i32>().unwrap()).collect();
            updates.push(update);
        });
    Input { predecessors, updates }
}
