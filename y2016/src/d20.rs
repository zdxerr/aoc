use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn next_u32(bytes: &mut impl Iterator<Item = u8>) -> Option<u32> {
    let mut number: Option<u32> = None;
    for b in bytes {
        let digit = b - b'0';
        if (0..=9).contains(&digit) {
            let n0 = number.get_or_insert_default();
            *n0 = *n0 * 10 + digit as u32;
        } else if number.is_some() {
            break;
        }
    }
    number
}

fn parse(input_path: &PathBuf) -> Result<Vec<(u32, u32)>, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let bytes = &mut reader.bytes().flatten().peekable();
    let mut ranges: Vec<(u32, u32)> = std::iter::repeat_with(|| (next_u32(bytes), next_u32(bytes)))
        .map_while(|(a, b)| a.and_then(|a| b.and_then(|b| Some((a, b)))))
        .collect();
    ranges.sort_unstable();
    Ok(ranges)
}

pub fn part1(input_path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    let ranges = parse(input_path)?;
    let mut ip = 0;

    for (start, end) in ranges {
        if ip < start {
            return Ok(ip);
        }
        if ip <= end {
            ip = end + 1;
        }
    }

    Err("no solution found".into())
}

pub fn part2(input_path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    let ranges = parse(input_path)?;
    let mut ip = 0;

    let mut count = 0;

    for (start, end) in ranges {
        if ip < start {
            count += start - ip;
            ip = end.saturating_add(1);
        } else if ip <= end {
            ip = end.saturating_add(1);
        }
    }
    Ok(count)
}
