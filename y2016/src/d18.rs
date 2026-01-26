use std::fs;
use std::path::PathBuf;

fn solve(input_path: &PathBuf, rows: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let mut row0: Vec<bool> = fs::read(input_path)?
        .trim_ascii()
        .iter()
        .map(|&c| c == b'^')
        .collect();
    let row_len = row0.len();
    let mut count = row0.iter().filter(|&&c| !c).count();

    let mut current_row = &mut row0;
    let mut previos_row = &mut vec![false; row_len];
    // println!();
    // println!(
    //     "{}   {count}",
    //     String::from_utf8_lossy(&previos_row).as_ref()
    // );

    for _ in 1..rows {
        for idx in 0..row_len {
            if idx == 0 {
                std::mem::swap(&mut previos_row, &mut current_row);
            }
            let triple = (
                if idx == 0 {
                    false
                } else {
                    previos_row[idx - 1]
                },
                previos_row[idx],
                if idx == row_len - 1 {
                    false
                } else {
                    previos_row[idx + 1]
                },
            );
            current_row[idx] = match triple {
                (false, _, true) | (true, _, false) => true,
                _ => {
                    count += 1;
                    false
                }
            };
        }
        // println!(
        //     "{}   {count}",
        //     String::from_utf8_lossy(&previos_row).as_ref()
        // );
    }

    Ok(count)
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, 40)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, 400_000)
}
