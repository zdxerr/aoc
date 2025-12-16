use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(file: File) -> impl Iterator<Item = [u64; 3]> {
    let reader = BufReader::new(file);
    reader.lines().map(|line| {
        let line = line.expect("unable to read line");
        let mut splitted = line.splitn(3, 'x');
        core::array::from_fn(|_| {
            splitted
                .next()
                .unwrap()
                .parse()
                .expect("unable to parse number")
        })
    })
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(parse(File::open(input_path)?)
        .map(|mut sides| {
            sides.sort();
            3 * sides[0] * sides[1] + 2 * sides[1] * sides[2] + 2 * sides[0] * sides[2]
        })
        .sum())
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(parse(File::open(input_path)?)
        .map(|mut sides| {
            sides.sort();
            sides[0] * sides[1] * sides[2] + 2 * sides[0] + 2 * sides[1]
        })
        .sum())
}
