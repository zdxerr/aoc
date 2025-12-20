use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    reader.bytes();

    let mut grid = [false; 1000 * 1000];

    let s: u64 = grid.iter().map(|v| if *v { 1 } else { 0 }).sum();
    Err("not implemented".into())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    reader.bytes();
    Err("not implemented".into())
}
