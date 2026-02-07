use crate::assembunny::run;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    run(input_path, &mut [7, 0, 0, 0], |_| true)
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    run(input_path, &mut [12, 0, 0, 0], |_| true)
}
