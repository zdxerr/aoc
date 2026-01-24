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

pub fn solve(
    input_path: &PathBuf,
    additional_discs: &[(u32, u32, u32)],
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut discs = parse_u32(input_path)?;
    for additional_disc in additional_discs {
        discs.push((
            discs.len() as u32 + 1,
            additional_disc.0,
            additional_disc.1,
            additional_disc.2,
        ));
    }
    discs.sort_by_key(|a| a.1);
    discs.reverse();

    let disc = discs[0];
    let start = disc.1 - (disc.0 + disc.3).rem_euclid(disc.1);

    't: for t0 in (start..).step_by(disc.1 as usize) {
        for disc in &discs[1..] {
            let start = disc.1 - (disc.0 + disc.3).rem_euclid(disc.1);
            if (t0 - start).rem_euclid(disc.1) != 0 {
                continue 't;
            }
        }
        return Ok(t0);
    }

    // disc / positions / time / position
    // 166 too low
    // 292158 too high
    Err("no solution found".into())
}

pub fn part1(input_path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    solve(input_path, &[])
}

pub fn part2(input_path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    solve(input_path, &[(11, 0, 0)])
}
