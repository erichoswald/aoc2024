fn main() {
    let sample = include_str!("../../inputs/sample04.txt");
    println!("Part 1 (sample): {}", part1(&sample));
    println!("Part 2 (sample): {}\n", part2(&sample));

    let input = include_str!("../../inputs/day04.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    count_words(input, find_xmas_at)
}

fn part2(input: &str) -> usize {
    count_words(input, find_x_mas_at)
}

fn count_words<F>(input: &str, count_at: F) -> usize
    where F: Fn(&[&str], i32, i32) -> usize
{
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).min().unwrap();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            count += count_at(&lines, x as i32, y as i32)
        }
    }
    count
}

fn find_xmas_at(lines: &[&str], x: i32, y: i32) -> usize {
    find_xmas_from(lines, x, y, 0, 1)
        + find_xmas_from(lines, x, y, 0, -1)
        + find_xmas_from(lines, x, y, 1, 0)
        + find_xmas_from(lines, x, y, -1, 0)
        + find_xmas_from(lines, x, y, 1, 1)
        + find_xmas_from(lines, x, y, 1, -1)
        + find_xmas_from(lines, x, y, -1, 1)
        + find_xmas_from(lines, x, y, -1, -1)
}

fn find_xmas_from(lines: &[&str], x: i32, y: i32, dx: i32, dy: i32) -> usize {
    if is_char(lines, x, y, 'X')
        && is_char(lines, x + dx, y + dy, 'M')
        && is_char(lines, x + 2 * dx, y + 2 * dy, 'A')
        && is_char(lines, x + 3 * dx, y + 3 * dy, 'S')
    {
        1
    } else {
        0
    }
}

fn find_x_mas_at(lines: &[&str], x: i32, y: i32) -> usize {
    if is_char(lines, x, y, 'A') && is_mas_at(lines, x, y, 1, 1) && is_mas_at(lines, x, y, 1, -1) {
        1
    } else {
        0
    }
}

fn is_mas_at(lines: &[&str], x: i32, y: i32, dx: i32, dy: i32) -> bool {
    is_char(lines, x + dx, y + dy, 'M') && is_char(lines, x - dx, y - dy, 'S')
        || is_char(lines, x - dx, y - dy, 'M') && is_char(lines, x + dx, y + dy, 'S')
}

fn is_char(lines: &[&str], x: i32, y: i32, ch: char) -> bool {
    char_at(lines, x, y) == Some(ch)
}

fn char_at(lines: &[&str], x: i32, y: i32) -> Option<char> {
    let line_index = usize::try_from(y).ok()?;
    let line = lines.get(line_index)?;
    let char_index = usize::try_from(x).ok()?;
    let char = line.chars().nth(char_index)?;
    Some(char)
}
