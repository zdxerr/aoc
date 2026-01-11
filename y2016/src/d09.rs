use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn next_usize(
    bytes: &mut std::iter::Flatten<std::io::Bytes<BufReader<fs::File>>>,
) -> (Option<usize>, usize) {
    let mut num = None;
    let mut counter = 0;
    for b in bytes {
        counter += 1;
        if (b'0'..=b'9').contains(&b) {
            let num = num.get_or_insert_default();
            *num = *num * 10 + (b - b'0') as usize;
        } else if num.is_some() {
            return (num, counter);
        }
    }
    (num, counter)
}

fn count(
    bytes: &mut std::iter::Flatten<std::io::Bytes<BufReader<fs::File>>>,
    mut len: usize,
    mul: usize,
    nested: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut counter = 0;

    while len > 0 {
        if let Some(b) = bytes.next() {
            len -= 1;
            match b {
                b'\n' | b' ' => (),
                b'A'..=b'Z' => counter += 1,
                b'(' => {
                    if let ((Some(len0), counter_len), (Some(mul0), counter_mul)) =
                        (next_usize(bytes), next_usize(bytes))
                    {
                        len -= len0 + counter_len + counter_mul;
                        if nested {
                            counter += count(bytes, len0, mul0, nested)?;
                        } else {
                            // If nested compression is not expected we can just skip these.
                            bytes.nth(len0 - 1).unwrap();
                            counter += len0 * mul0;
                        }
                    } else {
                        return Err("unable to parse compressions".into());
                    }
                }
                _ => return Err(format!("unexpected: {:?}", b as char).into()),
            }
        } else {
            break;
        }
    }
    Ok(counter * mul)
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let bytes = &mut reader.bytes().flatten();

    count(bytes, usize::MAX, 1, false)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let bytes = &mut reader.bytes().flatten();

    count(bytes, usize::MAX, 1, true)
}
