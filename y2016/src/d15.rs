use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn parse_u32(
    input_path: &PathBuf,
) -> Result<Vec<(u32, u32, u32, u32)>, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut numbers = Vec::new();
    let mut num = None;
    for b in reader.bytes().flatten() {
        if (b'0'..=b'9').contains(&b) {
            let num = num.get_or_insert_default();
            *num = *num * 10 + (b - b'0') as u32
        } else if let Some(_num) = num {
            numbers.push(_num);
            num = None;
        }
    }
    Ok(numbers
        .chunks_exact(4)
        .map(|a| (a[0], a[1], a[2], a[3]))
        .collect())
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // let _content = fs::read_to_string(input_path)?;

    let mut discs = parse_u32(input_path)?;
    discs.sort_by_key(|a| a.1);
    discs.reverse();
    println!();
    println!(" {:?}", discs);

    let first = discs[0];

    let start = (first.0 + first.3) % first.1;

    for idx in (start..1000).step_by(first.1 as usize) {
        println!("- {idx}");
    }

    // disc / positions / time / position

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
