use std::fs;
use std::path::PathBuf;

const STEPS: usize = 100;
const ON: u8 = b'#';
const OFF: u8 = b'.';
const NL: u8 = b'\n';

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid = fs::read(input_path)?;
    let width = grid
        .iter()
        .position(|c| c == &NL)
        .expect("newline not found");

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
    }
    Ok(grid.iter().filter(|c| **c == b'#').count())
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
