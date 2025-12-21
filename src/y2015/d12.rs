use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);

    let mut n: Option<u64> = None;
    let mut neg: i8 = 1;
    let mut sum = 0;
    // println!();
    for b in reader.bytes() {
        let b = b.unwrap();

        let d = b - b'0';

        if d < 10 {
            let n = n.get_or_insert(0);
            *n = *n * 10 + d as u64;
        } else if b == b'-' {
            neg = -1;
        } else {
            if let Some(v) = n {
                let v = v as i64 * neg as i64;
                // print!("{v} ");
                sum += v;
                n = None;
            }
            neg = 1;
        }
    }
    // dbg!(b'-', b'0');
    // println!();
    Ok(sum)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
