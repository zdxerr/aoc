use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const DURATION: u64 = 2503;

struct Reindeer {
    speed: u64,
    duration: u64,
    rest: u64,
}

impl Reindeer {
    fn new(line: &str) -> Self {
        let mut splitted = line.split(' ');
        Reindeer {
            speed: splitted.nth(3).unwrap().parse().unwrap(),
            duration: splitted.nth(2).unwrap().parse().unwrap(),
            rest: splitted.nth(6).unwrap().parse().unwrap(),
        }
    }
    fn distance(&self, time: u64) -> u64 {
        let total = self.duration + self.rest;
        let (q, r) = (time.div_euclid(total), time.rem_euclid(total));
        self.speed * (self.duration * q + self.duration.min(r))
    }
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    Ok(reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Reindeer::new(&line).distance(DURATION))
        .max()
        .unwrap())
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);

    let reindeers: Vec<Reindeer> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Reindeer::new(&line))
        .collect();

    let mut points = vec![0; reindeers.len()];
    let mut distances = vec![0; reindeers.len()];

    for time in 1..=DURATION {
        distances = reindeers
            .iter()
            .map(|reindeer| reindeer.distance(time))
            .collect();

        let mut max_indices = vec![];
        let mut max_value = None;
        for (idx, distance) in distances.iter().enumerate() {
            match max_value {
                None => {
                    max_value = Some(distance);
                    max_indices.push(idx);
                }
                Some(current_max) => match distance.partial_cmp(current_max) {
                    Some(std::cmp::Ordering::Greater) => {
                        max_value = Some(distance);
                        max_indices.clear();
                        max_indices.push(idx);
                    }
                    Some(std::cmp::Ordering::Equal) => {
                        max_indices.push(idx);
                    }
                    _ => {}
                },
            }
        }

        for max_idx in &max_indices {
            points[*max_idx] += 1;
        }
    }

    Ok(*points.iter().max().unwrap())
}
