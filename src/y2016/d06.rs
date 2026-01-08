use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut column_counter: Vec<[u64; 26]> = Vec::with_capacity(8);

    for row in reader.split(b'\n').flatten() {
        for (index, c) in row.iter().enumerate() {
            let c_index = (c - b'a') as usize;
            if column_counter.len() <= index {
                column_counter.insert(index, [0; 26]);
            }
            column_counter[index][c_index] += 1;
        }
    }
    Ok(column_counter
        .iter()
        .map(|counter| {
            let max_index = counter
                .iter()
                .enumerate()
                .max_by_key(|&(_, item)| item)
                .map(|(index, _)| index)
                .unwrap();
            (max_index as u8 + b'a') as char
        })
        .collect())
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut column_counter: Vec<[u64; 26]> = Vec::with_capacity(8);

    for row in reader.split(b'\n').flatten() {
        for (index, c) in row.iter().enumerate() {
            let c_index = (c - b'a') as usize;
            if column_counter.len() <= index {
                column_counter.insert(index, [0; 26]);
            }
            column_counter[index][c_index] += 1;
        }
    }
    Ok(column_counter
        .iter()
        .map(|counter| {
            let max_index = counter
                .iter()
                .enumerate()
                .min_by_key(|&(_, item)| item)
                .map(|(index, _)| index)
                .unwrap();
            (max_index as u8 + b'a') as char
        })
        .collect())
}
