use crate::assembunny::run;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    run(input_path, &mut [0; 4], |_| true)
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    run(input_path, &mut [0, 0, 1, 0], |_| true)
}
