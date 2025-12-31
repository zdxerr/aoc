use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

const STEPS: usize = 100;
const ON: u8 = b'#';
const OFF: u8 = b'.';
const NL: u8 = b'\n';

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut width = None;
    let mut grid: Vec<bool> = reader
        .bytes()
        .flatten()
        .enumerate()
        .filter_map(|(pos, c)| match c {
            ON => Some(true),
            OFF => Some(false),
            NL if width.is_none() => {
                width = Some(pos);
                None
            }
            _ => None,
        })
        .collect();
    let width = width.expect("newline not found");

    for _ in 0..STEPS {
        grid = (0..grid.len())
            .map(|index| {
                let neighboors = [
                    (index.rem_euclid(width) != 0 && index >= width).then_some(index - width - 1),
                    (index.rem_euclid(width) != width - 1 && index >= width)
                        .then_some(index - width + 1),
                    index.checked_sub(width),
                    (index.rem_euclid(width) != 0).then_some(index - 1),
                    (index.rem_euclid(width) != width - 1).then_some(index + 1),
                    (index.rem_euclid(width) != 0).then_some(index + width - 1),
                    index.checked_add(width),
                    (index.rem_euclid(width) != width - 1).then_some(index + width + 1),
                ]
                .iter()
                .flatten()
                .map(|index| grid.get(*index))
                .flatten()
                .filter(|c| **c)
                .count();

                match grid.get(index) {
                    Some(true) => [2, 3].contains(&neighboors),
                    Some(false) => neighboors == 3,
                    _ => panic!("index out of bounds"),
                }
            })
            .collect();
    }
    Ok(grid.iter().filter(|c| **c).count())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid = fs::read(input_path)?;
    let len = grid.len();
    let width = grid
        .iter()
        .position(|c| c == &NL)
        .expect("newline not found");

    grid[0] = ON;
    grid[width - 1] = ON;
    grid[len - width - 1] = ON;
    grid[len - 2] = ON;

    for _ in 0..STEPS {
        grid = (0..grid.len())
            .map(|index| {
                let neighboors = [
                    index.checked_sub(width + 1 + 1),
                    index.checked_sub(width + 1),
                    index.checked_sub(width - 1 + 1),
                    index.checked_sub(1),
                    index.checked_add(1),
                    index.checked_add(width - 1 + 1),
                    index.checked_add(width + 1),
                    index.checked_add(width + 1 + 1),
                ]
                .iter()
                .flatten()
                .map(|index| grid.get(*index))
                .flatten()
                .filter(|c| c == &&ON)
                .count();

                match grid.get(index) {
                    Some(&ON) => {
                        if [2, 3].contains(&neighboors) {
                            ON
                        } else {
                            OFF
                        }
                    }
                    Some(&OFF) => {
                        if neighboors == 3 {
                            ON
                        } else {
                            OFF
                        }
                    }
                    Some(c) => *c,
                    _ => panic!("index out of bounds"),
                }
            })
            .collect();
        grid[0] = ON;
        grid[width - 1] = ON;
        grid[len - width - 1] = ON;
        grid[len - 2] = ON;
    }
    Ok(grid.iter().filter(|c| **c == b'#').count())
}
