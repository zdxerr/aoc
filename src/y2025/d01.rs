use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fs, usize};

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let buff_reader = BufReader::new(fs::File::open(input_path)?);
    let mut s: i16 = 50;
    let mut count: usize = 0;
    for line in buff_reader.lines() {
        let line = line.expect("unable to read line");
        let c = line.chars().nth(0).expect("unable to read first char");
        let n = &line[1..].parse::<i16>().expect("unable to parse number");
        // println!("{s:#?} {c:#?} {n:#?}");
        s = match c {
            'R' => (s + n).rem_euclid(100),
            'L' => (s - n).rem_euclid(100),
            _ => panic!("unexpected first char"),
        };
        if s == 0 {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let buff_reader = BufReader::new(fs::File::open(&input_path)?);
    let mut s: i16 = 50;
    let mut count: usize = 0;
    for line in buff_reader.lines() {
        let line = line.expect("unable to read line");
        let c = line.chars().nth(0).expect("unable to read first char");
        let n = &line[1..].parse::<i16>().expect("unable to parse number");
        let w = match c {
            'R' => s + n,
            'L' => s - n,
            _ => panic!("unexpected first char"),
        };

        let a = w.div_euclid(100);
        let b = w.rem_euclid(100);
        // print!("{s:#?} {c:#?} {n:#?} {w:#?} {a} {b}");
        if c == 'L' && b == 0 {
            count += 1;
        }
        count += a.abs() as usize; // - 1;
        if c == 'L' && s == 0 {
            count -= 1;
        }
        // println!("  -- {count:?}");
        s = w.rem_euclid(100);
    }
    Ok(count)
}
