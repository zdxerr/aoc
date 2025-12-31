use crate::y2015::d04_md5::md5;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

fn solve(input_path: &PathBuf, mask: u32) -> Result<u64, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().trim().to_string();

    let next_number = AtomicU64::new(0);
    let result = AtomicU64::new(0);
    let num_workers = thread::available_parallelism()?.get();

    thread::scope(|s| {
        let prefix = &prefix;
        let next_number = &next_number;
        let result = &result;
        for _ in 0..num_workers {
            s.spawn(move || {
                loop {
                    if result.load(Ordering::Relaxed) != 0 {
                        return;
                    }
                    let number = next_number.fetch_add(1, Ordering::Relaxed);
                    let key = format!("{prefix}{number}");
                    let (word, _, _, _) = md5(&key);
                    if word & mask == 0 {
                        result.store(number, Ordering::Relaxed);
                    }
                }
            });
        }
    });

    Ok(result.load(Ordering::Relaxed))
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    solve(input_path, 0xFFFFF000)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    solve(input_path, 0xFFFFFF00)
}
