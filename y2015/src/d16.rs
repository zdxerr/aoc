use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let sue_facts: HashMap<&str, u8> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();

    reader
        .lines()
        .flatten()
        .find_map(|line| {
            let mut parts = line.split([' ', ':', ',']);
            let number: u64 = parts.nth(1).unwrap().parse().unwrap();

            let mut matched = true;

            while let (Some(compound), Some(amount)) = (parts.nth(1), parts.nth(1)) {
                let amount: u8 = amount.parse().unwrap();

                if let Some(actual_amount) = sue_facts.get(compound) {
                    if *actual_amount != amount {
                        matched = false;
                        break;
                    }
                }
            }
            if matched {
                return Some(number);
            }
            None
        })
        .ok_or("no match found".into())
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let sue_facts: HashMap<&str, u8> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();

    reader
        .lines()
        .flatten()
        .find_map(|line| {
            let mut parts = line.split([' ', ':', ',']);
            let number: u64 = parts.nth(1).unwrap().parse().unwrap();

            let mut matched = true;

            while let (Some(compound), Some(amount)) = (parts.nth(1), parts.nth(1)) {
                let amount: u8 = amount.parse().unwrap();

                if let Some(actual_amount) = sue_facts.get(compound) {
                    match compound {
                        "cats" | "trees" => {
                            if *actual_amount >= amount {
                                matched = false;
                                break;
                            }
                        }
                        "pomeranians" | "goldfish" => {
                            if *actual_amount <= amount {
                                matched = false;
                                break;
                            }
                        }
                        _ => {
                            if *actual_amount != amount {
                                matched = false;
                                break;
                            }
                        }
                    }
                }
            }
            if matched {
                return Some(number);
            }
            None
        })
        .ok_or("no match found".into())
}
