use std::fs;
use std::path::PathBuf;

const DISK_LENGHT: usize = 272;

fn bits_to_string(bits: &[bool]) -> String {
    bits.iter().map(|&b| if b { '1' } else { '0' }).collect()
}

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    let mut content: Vec<bool> = content.chars().map(|c| c == '1').collect();
    let mut idx = content.len();
    while content.len() < DISK_LENGHT {
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

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
