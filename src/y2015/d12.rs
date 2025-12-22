use std::fs::File;
use std::io::{self, BufReader, Bytes, PipeReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);

    let mut n: Option<u64> = None;
    let mut neg: i8 = 1;
    let mut sum = 0;
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
                sum += v;
                n = None;
            }
            neg = 1;
        }
    }
    Ok(sum)
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);

    let mut bytes = reader.bytes();

    fn scan_array(bytes: &mut Bytes<BufReader<File>>) -> i64 {
        let mut n: Option<u64> = None;
        let mut neg: i8 = 1;
        // let mut red: bool = false;
        let mut sum = 0;
        loop {
            match bytes.next() {
                Some(Ok(b'{')) => sum += scan_object(bytes),
                // Some(Ok(b'}')) => return if red { 0 } else { sum },
                Some(Ok(b'[')) => sum += scan_array(bytes),
                Some(Ok(b']')) => {
                    if let Some(v) = n {
                        let v = v as i64 * neg as i64;
                        // print!("{v} ");
                        sum += v;
                    }

                    return sum;
                }
                // Some(Ok(b'r')) => {
                //     if let (Some(Ok(b'e')), Some(Ok(b'd'))) = (bytes.next(), bytes.next()) {
                //         red = true;
                //     }
                // }
                Some(Ok(b'-')) => neg = -1,
                Some(Ok(b)) => {
                    let d = b - b'0';

                    if d < 10 {
                        let n = n.get_or_insert(0);
                        *n = *n * 10 + d as u64;
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
                _ => break,
            }
        }
        sum
    }

    fn scan_object(bytes: &mut Bytes<BufReader<File>>) -> i64 {
        let mut n: Option<u64> = None;
        let mut neg: i8 = 1;
        let mut red: bool = false;
        let mut sum = 0;
        loop {
            match bytes.next() {
                Some(Ok(b'{')) => sum += scan_object(bytes),
                Some(Ok(b'}')) => {
                    if let Some(v) = n {
                        let v = v as i64 * neg as i64;
                        // print!("{v} ");
                        sum += v;
                    }
                    return if red { 0 } else { sum };
                }
                Some(Ok(b'[')) => sum += scan_array(bytes),
                Some(Ok(b']')) => {
                    if let Some(v) = n {
                        let v = v as i64 * neg as i64;
                        // print!("{v} ");
                        sum += v;
                    }

                    return sum;
                }
                Some(Ok(b'r')) => {
                    if let (Some(Ok(b'e')), Some(Ok(b'd'))) = (bytes.next(), bytes.next()) {
                        red = true;
                    }
                }
                Some(Ok(b'-')) => neg = -1,
                Some(Ok(b)) => {
                    let d = b - b'0';

                    // print!("d {d} ");

                    if d < 10 {
                        let n = n.get_or_insert(0);
                        *n = *n * 10 + d as u64;
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
                _ => break,
            }
        }
        sum
    }
    Ok(scan_object(&mut bytes))
}
