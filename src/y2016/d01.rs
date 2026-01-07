use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut direction: i8 = 0;
    let mut position: (i64, i64) = (0, 0);
    reader.split(b',').flatten().for_each(|p| {
        let p = p.trim_ascii();
        direction = (direction
            + match p[0] {
                b'R' => 1,
                b'L' => -1,
                _ => panic!("invalid turn: {:?} {:?}", p, str::from_utf8(p)),
            })
        .rem_euclid(4);

        let number: i64 = p[1..].iter().fold(0, |n, c| n * 10 + (c - b'0') as i64);

        match direction {
            0 => position.1 += number,
            1 => position.0 += number,
            2 => position.1 -= number,
            3 => position.0 -= number,
            _ => panic!("invalid direction: {direction}"),
        }
    });

    Ok(position.0.abs() + position.1.abs())
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);

    let mut direction: i8 = 0;
    let mut position: (i64, i64) = (0, 0);
    let mut visited = HashSet::with_capacity(1000);

    for p in reader.split(b',').flatten() {
        let p = p.trim_ascii();
        direction = (direction
            + match p[0] {
                b'R' => 1,
                b'L' => -1,
                _ => panic!("invalid turn: {:?} {:?}", p, str::from_utf8(p)),
            })
        .rem_euclid(4);

        let number: i64 = p[1..].iter().fold(0, |n, c| n * 10 + (c - b'0') as i64);

        match direction {
            0 => {
                for y in position.1 + 1..=position.1 + number {
                    if !visited.insert((position.0, y)) {
                        return Ok(position.0.abs() + y.abs());
                    }
                }
                position.1 += number;
            }
            1 => {
                for x in position.0 + 1..=position.0 + number {
                    if !visited.insert((x, position.1)) {
                        return Ok(x.abs() + position.1.abs());
                    }
                }
                position.0 += number;
            }
            2 => {
                for y in position.1 - number..=position.1 - 1 {
                    if !visited.insert((position.0, y)) {
                        return Ok(position.0.abs() + y.abs());
                    }
                }
                position.1 -= number;
            }
            3 => {
                for x in position.0 - number..=position.0 - 1 {
                    if !visited.insert((x, position.1)) {
                        return Ok(x.abs() + position.1.abs());
                    }
                }
                position.0 -= number;
            }
            _ => panic!("invalid direction: {direction}"),
        }
    }
    Err("no location visited twice".into())
}
