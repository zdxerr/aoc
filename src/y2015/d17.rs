use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const FILL: u64 = 150;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let containers: Vec<u64> = reader
        .lines()
        .flatten()
        .map(|n| n.parse())
        .flatten()
        .collect();
    Ok((0..2_usize.pow(containers.len() as u32))
        .filter(|selection| {
            containers
                .iter()
                .enumerate()
                .filter(|(n, _)| (0b1_usize << n) & *selection != 0)
                .map(|(_, c)| c)
                .sum::<u64>()
                == FILL
        })
        .count())
}

pub fn part2(input_path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let containers: Vec<u64> = reader
        .lines()
        .flatten()
        .map(|n| n.parse())
        .flatten()
        .collect();

    Ok((0..2_usize.pow(containers.len() as u32))
        .filter(|selection| {
            containers
                .iter()
                .enumerate()
                .filter(|(n, _)| (0b1_usize << n) & *selection != 0)
                .map(|(_, c)| c)
                .sum::<u64>()
                == FILL
        })
        .fold((None, 0), |(min, counter), selection| {
            let container_count_new = selection.count_ones();
            match min {
                Some(container_count) if container_count_new > container_count => (min, counter),
                Some(container_count) if container_count_new == container_count => {
                    (min, counter + 1)
                }
                _ => (Some(container_count_new), 1),
            }
        })
        .1)
}
