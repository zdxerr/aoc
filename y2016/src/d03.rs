use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn parse(input_path: &PathBuf) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut numbers = vec![];
    let mut number: Option<u32> = None;

    reader.bytes().flatten().for_each(|b| {
        let b = b - b'0';
        match number {
            Some(this_number) if b < 10 => {
                number.replace(this_number * 10 + b as u32);
            }
            Some(this_number) => {
                numbers.push(this_number);
                number = None;
            }
            None if b < 10 => {
                number = Some(b as u32);
            }
            None => (),
        }
    });
    Ok(numbers)
}

#[inline]
fn valid(triangle: &mut [u32; 3]) -> bool {
    triangle.sort();
    triangle[0] + triangle[1] > triangle[2]
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let numbers = parse(input_path)?;
    Ok((0..numbers.len())
        .step_by(3)
        .map(|index| valid(&mut [numbers[index], numbers[index + 1], numbers[index + 2]]))
        .filter(|valid| *valid)
        .count())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let numbers = parse(input_path)?;
    Ok((0..numbers.len())
        .step_by(9)
        .flat_map(|index| {
            [
                valid(&mut [numbers[index], numbers[index + 3], numbers[index + 6]]),
                valid(&mut [numbers[index + 1], numbers[index + 4], numbers[index + 7]]),
                valid(&mut [numbers[index + 2], numbers[index + 5], numbers[index + 8]]),
            ]
        })
        .filter(|valid| *valid)
        .count())
}
