use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let _content = fs::read_to_string(input_path)?;
    let _reader = BufReader::new(fs::File::open(input_path)?);

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
