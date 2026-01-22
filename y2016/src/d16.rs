use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

const DISK_LENGHT: usize = 272;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let _reader = BufReader::new(fs::File::open(input_path)?);
    //
    //
    let mut content: Vec<bool> = content.chars().map(|c| c == '1').collect();
    // println!("{content}");
    let s: String = content.iter().map(|&b| if b { '1' } else { '0' }).collect();
    println!("{s:?}");
    // println!("{:0b}", x.concat());
    //
    //
    // let sl = &content[..];
    // let next: Vec<bool> = content.iter().rev().map(|b| !b).collect();
    // let new = content.extend_from_slice(&[false]).extend(next).collect();
    // let s: String = content.iter().map(|&b| if b { '1' } else { '0' }).collect();
    // println!("{s:?}");

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
