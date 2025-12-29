use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let input: usize = fs::read_to_string(input_path)?.trim().parse()?;
    let mut houses: Vec<usize> = vec![0; input.div_euclid(10)];

    for elve in 1..input.div_euclid(20) {
        for house in (elve..input.div_euclid(20)).step_by(elve) {
            houses[house] += elve * 10;
        }
    }
    Ok(houses
        .iter()
        .position(|presents| presents >= &input)
        .unwrap())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let input: usize = fs::read_to_string(input_path)?.trim().parse()?;
    let mut houses: Vec<usize> = vec![0; input.div_euclid(10)];

    for elve in 1..input.div_euclid(20) {
        for house in (elve..input.div_euclid(10).min(elve + elve * 50)).step_by(elve) {
            houses[house] += elve * 11;
        }
    }
    Ok(houses
        .iter()
        .position(|presents| presents >= &input)
        .unwrap())
}
