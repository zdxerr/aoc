use std::fs;
use std::mem;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let mut sequence: Vec<u8> = fs::read(input_path)?
        .iter()
        .map(|c| c - b'0')
        .filter(|v| *v < 10)
        .collect();

    let mut new = vec![];

    for _ in 0..40 {
        let mut i = sequence.iter();
        let mut count: u8 = 0;
        let mut last: &u8 = &u8::MAX;
        while let Some(c) = i.next() {
            if count >= 1 {
                if c == last {
                    count += 1;
                    continue;
                }
                new.push(count);
                new.push(*last);
            }
            last = c;
            count = 1;
        }
        if count >= 1 {
            new.push(count);
            new.push(*last);
        }
        mem::swap(&mut sequence, &mut new);
        new.clear();
    }
    Ok(sequence.len())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let mut sequence: Vec<u8> = fs::read(input_path)?
        .iter()
        .map(|c| c - b'0')
        .filter(|v| *v < 10)
        .collect();

    let mut new = vec![];

    for _ in 0..50 {
        let mut i = sequence.iter();
        let mut count: u8 = 0;
        let mut last: &u8 = &u8::MAX;
        while let Some(c) = i.next() {
            if count >= 1 {
                if c == last {
                    count += 1;
                    continue;
                }
                new.push(count);
                new.push(*last);
            }
            last = c;
            count = 1;
        }
        if count >= 1 {
            new.push(count);
            new.push(*last);
        }
        mem::swap(&mut sequence, &mut new);
        new.clear();
    }
    Ok(sequence.len())
}
