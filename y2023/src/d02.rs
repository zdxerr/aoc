use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut solution: usize = 0;

    'outer: for line in reader.lines() {
        let line = line.expect("unable to read line");
        let (game, rest) = line.split_once(':').unwrap();
        let (_, game) = game.split_once(' ').unwrap();

        for draw in rest.split(';') {
            for number_and_color in draw.split(",") {
                let (number, color) = number_and_color.trim().split_once(' ').unwrap();
                let number: u32 = number.parse().unwrap();
                match color {
                    "red" if number > RED => continue 'outer,
                    "green" if number > GREEN => continue 'outer,
                    "blue" if number > BLUE => continue 'outer,
                    _ => {}
                }
            }
        }
        solution += game.parse::<usize>().unwrap();
    }
    Ok(solution)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
