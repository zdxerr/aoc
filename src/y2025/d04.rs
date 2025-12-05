use std::fs;
use std::path::PathBuf;

const NL: u8 = b'\n';
const ROLL: u8 = b'@';

pub fn part1(input_path: &PathBuf) -> usize {
    let mut sum: usize = 0;
    let grid = fs::read(input_path).unwrap();

    let row_len = grid.iter().position(|c| c == &NL).unwrap();

    for index in 0..grid.len() {
        if grid[index] != ROLL {
            continue;
        }
        let adjacent_indices = [
            index.checked_sub(row_len + 2),
            index.checked_sub(row_len + 1),
            index.checked_sub(row_len),
            index.checked_sub(1),
            index.checked_add(1),
            index.checked_add(row_len),
            index.checked_add(row_len + 1),
            index.checked_add(row_len + 2),
        ];
        let adjacent_rolls = adjacent_indices
            .iter()
            .flatten()
            .filter(|index| **index < grid.len())
            .filter(|index| grid[**index] == ROLL)
            .count();

        if adjacent_rolls < 4 {
            sum += 1;
        }
    }
    sum
}

pub fn part2(input_path: &PathBuf) -> usize {
    let mut sum: usize = 0;
    let mut grid = fs::read(&input_path).unwrap();
    let row_len = grid.iter().position(|c| c == &NL).unwrap();

    loop {
        let last_sum = sum;
        for index in 0_usize..grid.len() {
            if grid[index] != ROLL {
                continue;
            }
            let adjacent_indices = [
                index.checked_sub(row_len + 2),
                index.checked_sub(row_len + 1),
                index.checked_sub(row_len),
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_add(row_len),
                index.checked_add(row_len + 1),
                index.checked_add(row_len + 2),
            ];
            let adjacent_rolls = adjacent_indices
                .iter()
                .flatten()
                .filter(|index| **index < grid.len())
                .filter(|index| grid[**index] == ROLL)
                .count();

            if adjacent_rolls < 4 {
                grid[index] = b'.';
                sum += 1;
            }
        }
        if last_sum == sum {
            break;
        }
    }

    sum
}
