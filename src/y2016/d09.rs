use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // let _content = fs::read_to_string(input_path)?;
    // let reader = BufReader::new(fs::File::open(input_path)?);
    let reader = BufReader::new(fs::File::open(r"input/y2016/d09/test.txt")?);
    println!();
    let bytes = &mut reader.bytes().flatten();

    loop {
        if let Some(b) = bytes.next() {
            println!("{}", b as char);
        } else {
            break;
        }
    }

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
