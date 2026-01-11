use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut position: i8 = 5;
    let mut code = String::new();

    reader.bytes().flatten().for_each(|c| {
        position += match c {
            b'U' => match position {
                4 | 5 | 6 | 7 | 8 | 9 => -3,
                _ => 0,
            },
            b'D' => match position {
                1 | 2 | 3 | 4 | 5 | 6 => 3,
                _ => 0,
            },
            b'L' => match position {
                2 | 3 | 5 | 6 | 8 | 9 => -1,
                _ => 0,
            },
            b'R' => match position {
                1 | 2 | 4 | 5 | 7 | 8 => 1,
                _ => 0,
            },
            b'\n' => {
                code.push((position as u8 + b'0') as char);
                0
            }
            _ => panic!("invalid instruction: {:?}", c as char),
        }
    });
    Ok(code)
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut position: i8 = 5;
    let mut code = String::new();

    reader.bytes().flatten().for_each(|c| {
        position += match c {
            b'U' => match position {
                3 | 13 => -2,
                6 | 7 | 8 | 10 | 11 | 12 => -4,
                _ => 0,
            },
            b'D' => match position {
                1 | 11 => 2,
                2 | 3 | 4 | 6 | 7 | 8 => 4,
                _ => 0,
            },
            b'L' => match position {
                3 | 4 | 6 | 7 | 8 | 9 | 11 | 12 => -1,
                _ => 0,
            },
            b'R' => match position {
                2 | 3 | 5 | 6 | 7 | 8 | 10 | 11 => 1,
                _ => 0,
            },
            b'\n' => {
                code.push(match position {
                    1..=9 => (position as u8 + b'0') as char,
                    _ => ((position - 10) as u8 + b'A') as char,
                });
                0
            }
            _ => panic!("invalid instruction: {:?}", c as char),
        }
    });
    Ok(code)
}
