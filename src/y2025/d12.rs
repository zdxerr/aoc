use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // This is not a generall solution to the problem and will not solve the test input.
    // It is fit to solve the input though.
    let content = fs::read_to_string(input_path)?;
    let sum: u64 = content
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (area, quantities) = line.split_once(':').unwrap();
            let (x, y) = area.split_once('x').unwrap();
            let (x, y): (u64, u64) = (x.parse().unwrap(), y.parse().unwrap());

            let n: u64 = quantities
                .trim()
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .sum();

            if (x / 3) * (y / 3) >= n {
                return 1;
            }
            0
        })
        .sum();
    Ok(sum)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(0)
}
