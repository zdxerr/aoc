use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let mut sum: u64 = 0;
    for bank in content.trim_ascii().split(|c| *c == b'\n') {
        let max = bank[..bank.len() - 1]
            .iter()
            .enumerate()
            .fold(
                (0, 0_u8),
                |max, (ind, &val)| if val > max.1 { (ind, val) } else { max },
            );
        let max2 = bank[max.0 + 1..].iter().max().unwrap();
        sum += str::from_utf8(&[max.1, *max2])?.parse::<u64>().unwrap();
    }
    Ok(sum)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let mut sum: usize = 0;
    let mut v: Vec<u8> = Vec::new();
    for bank in content.trim_ascii().split(|c| *c == b'\n') {
        let mut i = 0;
        v.clear();
        // println!(" {}", str::from_utf8(&bank).unwrap());
        for n in (0..12).rev() {
            let max =
                bank[i..bank.len() - n]
                    .iter()
                    .enumerate()
                    .fold(
                        (0, 0_u8),
                        |max, (ind, &val)| if val > max.1 { (ind, val) } else { max },
                    );

            v.push(max.1);
            // println!("   {n:3} {max:?}");
            i += max.0 + 1;
            // let max2 = bank[max.0 + 1..].iter().max().unwrap();
        }
        // println!(" {}", str::from_utf8(&v).unwrap());
        sum += str::from_utf8(&v)?.parse::<usize>().unwrap();
    }
    Ok(sum)
}
