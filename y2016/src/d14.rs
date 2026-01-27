use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use y2015::d04_md5::{md5, md5_from_bytes, md5_to_hex};

pub fn find_key(input_path: &PathBuf, stretch: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().to_string();
    // let prefix = "abc";

    let mut triples: Vec<(usize, u8)> = Vec::with_capacity(10000);
    let mut keys = HashSet::with_capacity(64);
    let mut max_key = 0;

    let mut number = 0;

    let hex = &mut [0u8; 32];

    while keys.len() < 64 || number < max_key + 1000 {
        let key = format!("{prefix}{number}");

        md5_to_hex(&md5(&key), hex);

        if stretch {
            for _ in 0..2016 {
                md5_to_hex(&md5_from_bytes(hex), hex);
            }
        }

        let bytes = &hex;
        let mut found_triple = false;
        for idx in 0..bytes.len() - 2 {
            let c = bytes[idx];
            if bytes[idx + 1] == c && bytes[idx + 2] == c {
                if idx + 4 < bytes.len() && bytes[idx + 3] == c && bytes[idx + 4] == c {
                    for triple in &triples {
                        if triple.1 == c && triple.0 < number && number <= triple.0 + 1000 {
                            keys.insert(triple.0);
                            max_key = max_key.max(triple.0);
                        }
                    }
                }
                if !found_triple {
                    triples.push((number, c));
                    found_triple = true;
                }
            }
        }
        number += 1;
    }

    let mut keys: Vec<_> = keys.into_iter().collect();
    keys.sort_unstable();

    if let Some(key) = keys.get(63) {
        Ok(*key)
    } else {
        Err("key not forund".into())
    }
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    find_key(input_path, false)
}
pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    find_key(input_path, true)
}
