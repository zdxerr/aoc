use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut bytes = reader.bytes();
    let mut count = 0;
    while let Some(Ok(b)) = bytes.next() {
        match b {
            b'"' => {
                count += 1;
                continue;
            }
            b'\\' => count += 1,
            _ => continue,
        };

        match bytes.next() {
            Some(Ok(b'x')) => {
                bytes.next();
                bytes.next();
                count += 2;
            }
            Some(Ok(b'"' | b'\\')) => (),
            b => {
                panic!("unexpected byte: {b:?}");
            }
        }
    }
    Ok(count)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut bytes = reader.bytes();
    let mut count = 0;
    while let Some(Ok(b)) = bytes.next() {
        match b {
            b'"' => {
                count += 2;
                continue;
            }
            b'\\' => count += 1,
            _ => continue,
        };

        match bytes.next() {
            Some(Ok(b'x')) => {
                bytes.next();
                bytes.next();
                // count += 2;
            }
            Some(Ok(b'"' | b'\\')) => count += 1,
            b => {
                panic!("unexpected byte: {b:?}");
            }
        }
    }
    Ok(count)
    // Err("not implemented".into())
}
