use std::fs;
use std::io::{BufRead, BufReader};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn part1(input_reader: BufReader<fs::File>) -> Option<usize> {
    let mut solution: usize = 0;

    'outer: for line in input_reader.lines() {
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
    Some(solution)
}
