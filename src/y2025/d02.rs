use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let mut sum: u64 = 0;
    for pair in content.split(|c| *c == b',') {
        let splitted: Vec<&[u8]> = pair.splitn(2, |v| *v == b'-').collect();
        let a: u64 = std::str::from_utf8(splitted[0])?.trim().parse()?;
        let b: u64 = std::str::from_utf8(splitted[1])?.trim().parse()?;

        for id in a..=b {
            let len = id.checked_ilog10().unwrap() + 1;
            let p = len.checked_div(2).unwrap();
            // if p == 0 {
            //     continue;
            // }
            let a = id.div_euclid(10_u64.pow(p));
            let b = id.rem_euclid(10_u64.pow(p));
            if a == b {
                sum += id;
            }
        }
    }
    Ok(sum)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    fn invalid(id: &usize) -> bool {
        let id = id.to_string();
        let len = id.len();

        for chunk_length in 1..=(len / 2) {
            if len % chunk_length != 0 {
                continue;
            }

            let mut invalid = true;

            for chunk_number in 1..(len / chunk_length) {
                if id[0..chunk_length]
                    != id[(chunk_number * chunk_length)..((chunk_number + 1) * chunk_length)]
                {
                    invalid = false;
                    break;
                }
            }
            if invalid {
                return true;
            }
        }
        false
    }

    let content = fs::read(input_path)?;

    let mut sum: usize = 0;
    for pair in content.split(|c| *c == b',') {
        let splitted: Vec<&[u8]> = pair.splitn(2, |v| *v == b'-').collect();
        let a: usize = std::str::from_utf8(splitted[0])?.trim().parse()?;
        let b: usize = std::str::from_utf8(splitted[1])?.trim().parse()?;

        for id in a..=b {
            if invalid(&id) {
                sum += id;
            }
        }
    }
    Ok(sum)
}
