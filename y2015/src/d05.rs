use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    Ok(reader
        .lines()
        .map(|res| res.unwrap())
        .filter(|line| {
            let mut twice = false;
            let mut aeiou: u64 = 0;
            for idx in 0..line.len() {
                match line.get(idx..idx + 2) {
                    Some("ab" | "cd" | "pq" | "xy") => {
                        return false;
                    }
                    Some(s) => {
                        twice |= {
                            let mut chars = s.chars();
                            chars.next() == chars.next()
                        }
                    }
                    _ => (),
                }
                aeiou += match line.get(idx..idx + 1) {
                    Some("a" | "e" | "i" | "o" | "u") => 1,
                    _ => 0,
                }
            }
            aeiou >= 3 && twice
        })
        .count())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    Ok(reader
        .lines()
        .map(|res| res.unwrap())
        .filter(|line| {
            let mut triplet = false;
            let mut double = false;
            for idx in 0..line.len() {
                triplet |= match line.get(idx..idx + 3) {
                    Some(s) => {
                        let mut chars = s.chars();
                        let (a, _, c) = (chars.next(), chars.next(), chars.next());
                        a == c
                    }
                    _ => false,
                };

                if let Some(s0) = line.get(idx..idx + 2) {
                    for idx in idx + 2..line.len() {
                        if let Some(s1) = line.get(idx..idx + 2) {
                            if s0 == s1 {
                                double = true;
                                break;
                            }
                        }
                    }
                }

                if triplet && double {
                    return true;
                }
            }
            false
        })
        .count())
}
