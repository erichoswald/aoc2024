use aoc2024::grid::{Grid, GridMove, GridPos};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let sample = include_str!("../../inputs/sample20.txt");
    println!("Sample");
    solve(&sample, 2, 10);
    solve(&sample, 20, 70);

    let input = include_str!("../../inputs/day20.txt");
    println!("\nPuzzle");
    solve(&input, 2, 100);
    solve(&input, 20, 100);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Path {
    pos: GridPos,
    time_to_end: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time_to_end.cmp(&self.time_to_end)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str, cheat_move_count: usize, min_savings: usize) {
    let track = Grid::parse_from(input);
    let mut times_to_end = HashMap::new();
    let start_pos = *track.cell_positions_with('S').iter().next().unwrap();
    let end_pos = *track.cell_positions_with('E').iter().next().unwrap();
    find_time_to_end(&track, start_pos, end_pos, &mut times_to_end);
    let cheat_moves = build_cheat_moves(cheat_move_count);
    let savings = get_cheat_savings(&track, &times_to_end, &cheat_moves);
    let savings_count = savings.iter().filter(|(_, saving)| **saving >= min_savings).count();
    println!("Number of cheats saving at least {min_savings} picoseconds: {savings_count}");
}

fn find_time_to_end(
    track: &Grid<char>,
    start_pos: GridPos,
    end_pos: GridPos,
    times_to_end: &mut HashMap<GridPos, usize>,
) {
    for (pos, _) in track.iter() {
        times_to_end.insert(*pos, usize::MAX);
    }
    let mut queue = BinaryHeap::new();
    queue.push(Path { pos: end_pos, time_to_end: 0 });
    while let Some(Path { time_to_end, pos }) = queue.pop() {
        if time_to_end > times_to_end[&pos] {
            continue;
        }
        times_to_end.insert(pos, time_to_end);
        if pos != start_pos {
            for n in pos.neighbours_4() {
                if !is_wall(track, n) {
                    queue.push(Path { pos: n, time_to_end: time_to_end + 1 });
                }
            }
        }
    }
}

fn get_cheat_savings(
    track: &Grid<char>,
    times_to_end: &HashMap<GridPos, usize>,
    cheat_moves: &Vec<GridMove>
) -> HashMap<(GridPos, GridPos), usize> {
    let mut savings = HashMap::new();
    for (pos, time) in times_to_end {
        if *time < usize::MAX {
            store_cheat_savings(track, times_to_end, cheat_moves, pos, &mut savings);
        }
    }
    savings
}

fn store_cheat_savings(
    track: &Grid<char>,
    times_to_end: &HashMap<GridPos, usize>,
    cheat_moves: &Vec<GridMove>,
    pos: &GridPos,
    savings: &mut HashMap<(GridPos, GridPos), usize>
) {
    for cheat_move in cheat_moves {
        let cheat_pos = pos.add(*cheat_move);
        if !is_wall(track, cheat_pos) {
            let saving = cheat_move_saving(times_to_end, *pos, cheat_pos);
            if saving > 0 {
                savings.insert((*pos, cheat_pos), saving);
            }
        }
    }
}

fn cheat_move_saving(times_to_end: &HashMap<GridPos, usize>, from: GridPos, to: GridPos) -> usize {
    let from_time = times_to_end[&from];
    if let Some(to_time) = times_to_end.get(&to) {
        let d = from.move_to(to).distance();
        if to_time + d < from_time {
            return from_time - to_time - d
        }
    }
    0
}

fn build_cheat_moves(cheat_move_count: usize) -> Vec<GridMove> {
    let mut moves = Vec::with_capacity(cheat_move_count * cheat_move_count);
    let my = cheat_move_count as i32;
    for dy in -my..=my {
        let mx = my - dy.abs();
        for dx in -mx..=mx {
            moves.push(GridMove::new(dy, dx))
        }
    }
    moves
}

fn is_wall(track: &Grid<char>, pos: GridPos) -> bool {
    track.at(pos) == Some('#')
}
