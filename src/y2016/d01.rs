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

// Then, you notice the instructions continue on the back of the Recruiting Document. Easter Bunny HQ is actually at the first location you visit twice.

// For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4 blocks away, due East.

// How many blocks away is the first location you visit twice?
pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    // let _content = fs::read_to_string(input_path)?;
    let reader = BufReader::new(fs::File::open(input_path)?);

    // let s: Vec<_> = "R5, L5, R5, R3".bytes().collect();
    // let s: Vec<_> = "R2, R2, R2".bytes().collect();
    // let s: Vec<_> = "R2, L3".bytes().collect();
    // let s: Vec<_> = "L1, L1, L1, L1, L1, L1".bytes().collect();
    let s: Vec<_> = "R8, R4, R4, R8".bytes().collect();

    let mut direction: i8 = 0;
    // let mut distance: i64 = 0;
    let mut position: (i64, i64) = (0, 0);
    // reader
    // .split(b',')
    // .flatten()
    s.split(|c| *c == b',')
        // .map(|c| c.trim_ascii())
        .for_each(|p| {
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
            println!(
                "{}, {direction} {number} {position:?}",
                str::from_utf8(p).unwrap()
            );
        });

    Ok(position.0.abs() + position.1.abs())
}
