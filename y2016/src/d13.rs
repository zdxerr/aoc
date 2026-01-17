use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;

#[inline]
fn space(x: &usize, y: &usize, number: &usize) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + number)
        .count_ones()
        .rem_euclid(2)
        == 0
}

fn solve(input_path: &PathBuf, part1: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let number: usize = fs::read_to_string(input_path)?.trim().parse()?;
    let mut queue = VecDeque::with_capacity(100);
    let mut visited = HashSet::with_capacity(1000);

    queue.push_front((1, 1, 0));

    while let Some((x, y, s)) = queue.pop_front() {
        if part1 {
            if x == 31 && y == 39 {
                return Ok(s);
            }
        } else if s > 50 {
            continue;
        }
        if !visited.insert((x, y)) {
            continue;
        }

        queue.extend(
            [
                Some((x, y + 1)),
                if y > 0 { Some((x, y - 1)) } else { None },
                Some((x + 1, y)),
                if x > 0 { Some((x - 1, y)) } else { None },
            ]
            .iter()
            .flatten()
            .filter(|(x, y)| space(x, y, &number))
            .map(|(x, y)| (*x, *y, s + 1)),
        );
    }

    Ok(visited.len())
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, true)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, false)
}
