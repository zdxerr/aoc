use std::fs;
use std::path::PathBuf;

fn bits_to_string(bits: &[bool]) -> String {
    bits.iter().map(|&b| if b { '1' } else { '0' }).collect()
}

pub fn calculate_checksum(
    input_path: &PathBuf,
    disk_length: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut content: Vec<bool> = Vec::with_capacity(disk_length);
    content.extend(
        fs::read_to_string(input_path)?
            .trim()
            .chars()
            .map(|c| c == '1'),
    );
    let mut idx = content.len();
    while content.len() < disk_length {
        if idx == content.len() {
            content.push(false);
        } else {
            content.push(!content[idx]);
            if idx == 0 {
                idx = content.len();
                continue;
            }
        }
        idx -= 1;
    }
    let mut checksum = content;
    while checksum.len().rem_euclid(2) == 0 {
        checksum = checksum
            .chunks_exact(2)
            .map(|pair| if pair[0] == pair[1] { true } else { false })
            .collect();
    }

    Ok(bits_to_string(&checksum))
}

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    calculate_checksum(input_path, 272)
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    // The second disk you have to fill has length 35651584. Again using the initial state in
    // your puzzle input, what is the correct checksum for this disk?
    calculate_checksum(input_path, 35651584)
}
