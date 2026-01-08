use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);

    Ok(reader
        .split(b'\n')
        .flatten()
        .filter(|address| {
            let mut abba_supernet = false;
            let mut abba_hypernet = false;
            let mut hypernet = false;
            for idx in 0..address.len() - 3 {
                if address[idx] == b'[' {
                    hypernet = true;
                } else if address[idx] == b']' {
                    hypernet = false;
                }
                if address[idx] == address[idx + 3]
                    && address[idx + 1] == address[idx + 2]
                    && address[idx + 0] != address[idx + 1]
                {
                    if hypernet {
                        abba_hypernet = true;
                    } else {
                        abba_supernet = true;
                    }
                }
            }
            abba_supernet && !abba_hypernet
        })
        .count())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();

    Ok(reader
        .split(b'\n')
        .flatten()
        .filter(|address| {
            abas.clear();
            babs.clear();
            let mut hypernet = false;
            for idx in 0..address.len() - 2 {
                if address[idx] == b'[' {
                    hypernet = true;
                } else if address[idx] == b']' {
                    hypernet = false;
                }

                if address[idx] == address[idx + 2] && address[idx] != address[idx + 1] {
                    if hypernet {
                        babs.insert((address[idx + 1], address[idx]));
                    } else {
                        abas.insert((address[idx], address[idx + 1]));
                    }
                }
            }
            !abas.is_disjoint(&babs)
        })
        .count())
}
