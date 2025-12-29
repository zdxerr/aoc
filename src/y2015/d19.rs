use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let reader = BufReader::new(fs::File::open(input_path)?);
    // let mut lines = reader.lines().flatten();

    let mut lines = content.lines();
    let x: HashMap<&str, &str> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.rsplit_once(" => "))
        .flatten()
        .collect();
    // let d = lines.next().unwrap();
    dbg!(x);

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
