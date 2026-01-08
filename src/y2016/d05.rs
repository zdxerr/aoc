use crate::y2015::d04_md5::md5;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::thread;

const MASK: u32 = 0xFFFFF000;

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().to_string();
    let next_number = AtomicU64::new(0);
    let num_workers = thread::available_parallelism()?.get();
    let mut password: u32 = 0;
    let found = AtomicBool::new(false);

    thread::scope(|s| {
        let (tx, rx): (mpsc::Sender<u32>, mpsc::Receiver<u32>) = mpsc::channel();
        let next_number = &next_number;
        let prefix = &prefix;
        for _ in 0..num_workers {
            let tx = tx.clone();
            let found = &found;
            s.spawn(move || {
                let mut key = String::with_capacity(20);
                key.push_str(prefix);
                let prefix_len = prefix.len();
                loop {
                    if found.load(Ordering::Relaxed) {
                        break;
                    }
                    let number = next_number.fetch_add(1, Ordering::Relaxed).to_string();
                    key.replace_range(prefix_len.., &number);
                    let (word, _, _, _) = md5(&key);
                    if word & MASK == 0 {
                        if let Err(_) = tx.send((word & 0x00000F00) >> 8) {
                            break;
                        }
                    }
                }
            });
        }
        drop(tx);
        for code in rx.iter().take(8) {
            password = password << 4 | code;
        }
        found.store(true, Ordering::Relaxed);
    });
    Ok(format!("{password:x}"))
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().to_string();

    let next_number = AtomicU64::new(0);
    let num_workers = thread::available_parallelism()?.get();
    let mut password: u32 = 0;
    let found = AtomicBool::new(false);

    thread::scope(|s| {
        let (tx, rx): (mpsc::Sender<(usize, u32)>, mpsc::Receiver<(usize, u32)>) = mpsc::channel();
        let next_number = &next_number;
        let prefix = &prefix;
        let found = &found;
        for _ in 0..num_workers {
            let tx = tx.clone();
            s.spawn(move || {
                let mut key = prefix.to_string();
                let prefix_len = prefix.len();
                loop {
                    if found.load(Ordering::Relaxed) {
                        break;
                    }
                    let number = next_number.fetch_add(1, Ordering::Relaxed).to_string();
                    key.replace_range(prefix_len.., &number);
                    let (word, _, _, _) = md5(&key);
                    if word & MASK == 0 {
                        let position = ((word & 0x00000F00) >> 8) as usize;
                        if (0..8).contains(&position) {
                            let code = ((word & 0x000000F0) >> 4) as u32;
                            if let Err(_) = tx.send((position, code)) {
                                break;
                            }
                        }
                    }
                }
            });
        }
        drop(tx);
        let mut positions_found: u8 = 0;
        for (position, code) in rx {
            if (1 << position) & positions_found == 0 {
                password |= code << (28 - (position * 4));
                positions_found |= 1 << position;
            }
            if positions_found == 0b11111111 {
                found.store(true, Ordering::Relaxed);
                break;
            }
        }
    });
    Ok(format!("{:08x}", password))
}
