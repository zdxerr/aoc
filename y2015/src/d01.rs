use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    Ok(reader
        .bytes()
        .map(|c| match c {
            Ok(b'(') => 1,
            Ok(b')') => -1,
            _ => 0,
        })
        .sum())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    Ok(reader
        .bytes()
        .scan(0, |acc, c| {
            *acc += match c {
                Ok(b'(') => 1,
                Ok(b')') => -1,
                _ => 0,
            };
            Some(*acc)
        })
        .position(|v| v == -1)
        .expect("-1 not found")
        + 1)
}
