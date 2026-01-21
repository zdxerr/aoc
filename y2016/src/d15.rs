use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn next_usize(
    bytes: &mut std::iter::Flatten<std::io::Bytes<BufReader<fs::File>>>,
) -> Option<usize> {
    let mut num = None;
    for b in bytes {
        if (b'0'..=b'9').contains(&b) {
            let num = num.get_or_insert_default();
            *num = *num * 10 + (b - b'0') as usize;
        } else if num.is_some() {
            return num;
        }
    }
    num
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // let _content = fs::read_to_string(input_path)?;
    let reader = BufReader::new(fs::File::open(input_path)?);

    let mut bytes = reader.bytes().flatten();
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));
    println!(" {:?}", next_usize(&mut bytes));

    let v = vec![1, 2, 3, 4, 5];

    // disc / positions / time / position

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
