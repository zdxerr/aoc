use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

struct UnsignedIterator {
    reader: std::iter::Flatten<std::io::Bytes<BufReader<fs::File>>>,
}

impl UnsignedIterator {
    fn new(file: fs::File) -> Self {
        UnsignedIterator {
            reader: BufReader::new(file).bytes().flatten(),
        }
    }
}

impl Iterator for UnsignedIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut number: Option<Self::Item> = None;
        while let Some(byte) = self.reader.next() {
            if matches!(byte, b'0'..=b'9') {
                let digit = (byte - b'0') as Self::Item;
                if let Some(number_internal) = number {
                    number = Some(number_internal * 10 + digit);
                } else {
                    number = Some(digit);
                }
            } else if number.is_some() {
                return number;
            }
        }
        None
    }
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let numbers: Vec<_> = UnsignedIterator::new(fs::File::open(input_path)?).collect();
    let mut left: Vec<_> = numbers.iter().step_by(2).collect();
    let mut right: Vec<_> = numbers.iter().skip(1).step_by(2).collect();

    left.sort_unstable();
    right.sort_unstable();

    Ok(left.iter().zip(right).map(|(a, b)| a.abs_diff(*b)).sum())
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let numbers: Vec<_> = UnsignedIterator::new(fs::File::open(input_path)?).collect();
    let mut left: Vec<_> = numbers.iter().step_by(2).collect();
    let mut right: Vec<_> = numbers.iter().skip(1).step_by(2).collect();

    left.sort_unstable();
    right.sort_unstable();

    let mut index_left = 0;
    let mut index_right = 0;
    let mut counter = 0;

    loop {
        if let (Some(a), Some(b)) = (left.get(index_left), right.get(index_right)) {
            if a < b {
                index_left += 1;
            } else if a > b {
                index_right += 1;
            }
            if a == b {
                let skip_left = left[index_left..]
                    .iter()
                    .take_while(|&number| number == a)
                    .count();
                let skip_right = right[index_right..]
                    .iter()
                    .take_while(|&number| number == a)
                    .count();
                counter += **a as usize * skip_left * skip_right;
                index_left += skip_left;
                index_right += skip_right;
            }
        } else {
            break;
        }
    }

    Ok(counter)
}
