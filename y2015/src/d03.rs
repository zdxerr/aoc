use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut posisitons: HashSet<(i64, i64)> = reader
        .bytes()
        .scan((0, 0), |(x, y), b| {
            match b.unwrap() {
                b'^' => *y -= 1,
                b'v' => *y += 1,
                b'<' => *x -= 1,
                b'>' => *x += 1,
                _ => {}
            }
            Some((*x, *y))
        })
        .collect();
    posisitons.insert((0, 0));
    Ok(posisitons.len())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut posisitons: HashSet<(i64, i64)> = reader
        .bytes()
        .scan(((0, 0), (0, 0)), |((x, y), (x0, y0)), b| {
            match b.unwrap() {
                b'^' => *y -= 1,
                b'v' => *y += 1,
                b'<' => *x -= 1,
                b'>' => *x += 1,
                _ => {}
            }
            std::mem::swap(x, x0);
            std::mem::swap(y, y0);
            Some((*x, *y))
        })
        .collect();
    posisitons.insert((0, 0));
    Ok(posisitons.len())
}
