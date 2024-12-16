use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use aoc2024::grid::{Grid, GridMove, GridPos, EAST, NORTH, SOUTH, WEST};

fn main() {
    let sample_1 = include_str!("../../inputs/sample16_1.txt");
    let sample_2 = include_str!("../../inputs/sample16_2.txt");
    let input = include_str!("../../inputs/day16.txt");

    println!("Part 1 (sample 1): {}", part1(&sample_1));
    println!("Part 1 (sample 2): {}", part1(&sample_2));
    println!("Part 1: {}", part1(&input));

    // println!("Part 2 (sample): {}\n", part2(&sample));
    // println!("Part 2: {}", part2(&input));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Path {
    score: usize,
    pos: GridPos,
    direction: GridMove,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> usize {
    let maze = Grid::parse_from(input);
    let start_pos = *maze.cell_positions_with('S').iter().next().unwrap();
    let end_pos = *maze.cell_positions_with('E').iter().next().unwrap();
    let mut lowest_score = HashMap::new();
    for (pos, ch) in maze.iter() {
        let score = if *ch == '#' { 0 } else { usize::MAX };
        lowest_score.insert((*pos, NORTH), score);
        lowest_score.insert((*pos, SOUTH), score);
        lowest_score.insert((*pos, EAST), score);
        lowest_score.insert((*pos, WEST), score);
    }
    let mut queue = BinaryHeap::new();
    queue.push(Path { score: 0, pos: start_pos, direction: EAST });
    while let Some(Path { score, pos, direction }) = queue.pop() {
        if pos == end_pos {
            return score;
        }
        if score >= lowest_score[&(pos, direction)] {
            continue;
        }
        lowest_score.insert((pos, direction), score);
        queue.push(Path { score: score + 1, pos: pos.add(direction), direction });
        queue.push(Path { score: score + 1000, pos, direction: direction.turn_left() });
        queue.push(Path { score: score + 1000, pos, direction: direction.turn_right() });
    }
    0
}
