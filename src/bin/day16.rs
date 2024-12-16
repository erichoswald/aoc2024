use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use aoc2024::grid::{Grid, GridMove, GridPos, EAST, NORTH, SOUTH, WEST};

fn main() {
    let sample_1 = include_str!("../../inputs/sample16_1.txt");
    let sample_2 = include_str!("../../inputs/sample16_2.txt");
    let input = include_str!("../../inputs/day16.txt");

    println!("Sample 1");
    solve(&sample_1);
    println!("\nSample 2");
    solve(&sample_2);
    println!("\nPuzzle");
    solve(&input);
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

fn solve(input: &str) {
    let maze = Grid::parse_from(input);
    let mut lowest_score = HashMap::new();
    let start_pos = *maze.cell_positions_with('S').iter().next().unwrap();
    let end_pos = *maze.cell_positions_with('E').iter().next().unwrap();
    let score = find_lowest_score(&maze, start_pos, end_pos, &mut lowest_score);
    println!("Lowest score: {}", score);
    let best_spot_count = count_best_spots(&maze, start_pos, end_pos, &lowest_score, score);
    println!("Best spots: {}", best_spot_count);
}

fn find_lowest_score(
    maze: &Grid<char>,
    start_pos: GridPos,
    end_pos: GridPos,
    lowest_score: &mut HashMap<(GridPos, GridMove), usize>,
) -> usize {
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
        if score > lowest_score[&(pos, direction)] {
            continue;
        }
        lowest_score.insert((pos, direction), score);
        if pos != end_pos {
            queue.push(Path { score: score + 1, pos: pos.add(direction), direction });
            queue.push(Path { score: score + 1000, pos, direction: direction.turn_left() });
            queue.push(Path { score: score + 1000, pos, direction: direction.turn_right() });
        }
    }
    lowest_score.iter()
        .filter(|(&key, _)| key.0 == end_pos)
        .map(|(_, &score)| score)
        .min().unwrap()
}

fn count_best_spots(
    maze: &Grid<char>,
    start_pos: GridPos,
    end_pos: GridPos,
    lowest_score: &HashMap<(GridPos, GridMove), usize>,
    best_score: usize,
) -> usize {
    let mut best_spots = HashSet::new();
    is_on_best_path(maze, start_pos, end_pos, 0, EAST, lowest_score, best_score, &mut best_spots);
    best_spots.len()
}

fn is_on_best_path(
    maze: &Grid<char>,
    cur_pos: GridPos,
    end_pos: GridPos,
    score: usize,
    direction: GridMove,
    lowest_score: &HashMap<(GridPos, GridMove), usize>,
    best_score: usize,
    best_spots: &mut HashSet<GridPos>,
) -> bool{
    if cur_pos == end_pos && score == best_score {
        best_spots.insert(cur_pos);
        return true
    } else if score == lowest_score[&(cur_pos, direction)] {
        let a = is_on_best_path(maze, cur_pos.add(direction), end_pos, score + 1, direction, lowest_score, best_score, best_spots);
        let b = is_on_best_path(maze, cur_pos, end_pos, score + 1000, direction.turn_left(), lowest_score, best_score, best_spots);
        let c = is_on_best_path(maze, cur_pos, end_pos, score + 1000, direction.turn_right(), lowest_score, best_score, best_spots);
        if a || b || c {
            best_spots.insert(cur_pos);
            return true
        }
    }
    false
}
