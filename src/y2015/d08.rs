use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut bytes = reader.bytes();
    println!();
    let mut count = 0;
    while let Some(Ok(b)) = bytes.next() {
        if b.is_ascii_whitespace() {
            continue;
        }

        count += match b {
            b'"' => 1,
            b'\\' => match bytes.next() {
                Some(Ok(b'"')) => 1,
                Some(Ok(b'x')) => 1,
                _ => 0,
            },
            _ => 0,
        };

        let c = b as char;
        print!("{c} ");
    }
    println!();
    Ok(count)
    // Err("not implemented".into())
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    Err("not implemented".into())
}
