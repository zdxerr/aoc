use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use y2015::d04_md5::{md5, md5_from_bytes, md5_to_hex};

pub fn find_key(input_path: &PathBuf, stretch: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().to_string();
    // let prefix = "abc";

    let triples = Arc::new(Mutex::new(Vec::<(usize, u8)>::with_capacity(10000)));
    let keys = Arc::new(Mutex::new(HashSet::with_capacity(64)));
    let max_key = Arc::new(AtomicUsize::new(0));
    let number = Arc::new(AtomicUsize::new(0));

    let num_workers = thread::available_parallelism()?.get();

    thread::scope(|s| {
        for _ in 0..num_workers {
            let prefix = &prefix;
            let triples = triples.clone();
            let keys = keys.clone();
            let max_key = max_key.clone();
            let number = number.clone();
            s.spawn(move || {
                let hex = &mut [0u8; 32];
                while keys.lock().unwrap().len() < 64
                    || number.load(Ordering::Relaxed) < (max_key.load(Ordering::Relaxed) + 1000)
                {
                    let number = number.fetch_add(1, Ordering::Relaxed);
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
                            let mut triples = triples.lock().unwrap();
                            if idx + 4 < bytes.len() && bytes[idx + 3] == c && bytes[idx + 4] == c {
                                for triple in triples.iter() {
                                    if triple.1 == c
                                        && triple.0 < number
                                        && number <= triple.0 + 1000
                                    {
                                        let mut keys = keys.lock().unwrap();
                                        keys.insert(triple.0);

                                        max_key.store(
                                            max_key.fetch_max(triple.0, Ordering::Relaxed),
                                            Ordering::Relaxed,
                                        );
                                        // let max_key = max_key.store(val, Ordering::Relaxed); max_key.get_mut();
                                        // max_key = *max_key.max(&triple.0);
                                    }
                                }
                            }
                            if !found_triple {
                                triples.push((number, c));
                                found_triple = true;
                            }
                        }
                    }
                }
            });
        }
    });

    let mut keys: Vec<_> = Arc::try_unwrap(keys)
        .unwrap()
        .into_inner()
        .unwrap()
        .into_iter()
        .collect();
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
