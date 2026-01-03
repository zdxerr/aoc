use std::fs;
use std::path::PathBuf;

fn pos(row: u64, column: u64) -> u64 {
    // codes are positioned in diagonal order in the table
    // calculate the row and column of the start of this diagonal line of positions
    let row0 = row + column - 1;
    // use Gauss sum to calculate the position of row0, column0 since each diagonal line contains <row> elements
    let pos0 = ((row0 - 1) * (row0 - 1 + 1)) / 2;
    pos0 + column
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    let mut numbers = vec![];
    let mut num = None;
    for c in content.bytes() {
        if c.is_ascii_digit() {
            match num {
                Some(ref mut num) => *num = *num * 10 + (c - b'0') as u64,
                None => num = Some((c - b'0') as u64),
            }
        } else if let Some(number) = num {
            numbers.push(number);
            num = None;
        }
    }

    let [row, column] = numbers
        .try_into()
        .expect("expected row and column not in input");

    let pos = pos(row, column);

    // Modular exponentiation: https://en.wikipedia.org/wiki/Modular_exponentiation
    // (a * b) mod m = [(a mod m) * (b mod m)] mod m
    let mut c = 20151125_u64;
    for _ in 0..(pos - 1) {
        c = (c * 252533_u64) % 33554393;
    }

    let mut first = 20151125_u64;
    for _ in 2..=pos {
        first = first.strict_mul(252533_u64).rem_euclid(33554393);
    }

    Ok(first)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(0)
}
