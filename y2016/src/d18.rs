use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn _print_traps(traps: &[bool], row_len: &usize) {
    for idx in (0..traps.len()).step_by(*row_len) {
        println!(
            "{}",
            traps[idx..traps.len().min(idx + row_len)]
                .iter()
                .map(|trap| if *trap { '^' } else { '.' })
                .collect::<String>()
        );
    }
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let row0 = fs::read_to_string(input_path)?.trim().to_string();
    let rows = 40;

    // let row0 = ".^^.^.^^^^";
    let row_len = row0.len();
    let mut traps: Vec<bool> = row0.bytes().map(|c| c == b'^').collect();

    for idx in row_len..row_len * rows {
        let a = if idx.rem_euclid(row_len) == 0 {
            false
        } else {
            traps[idx - row_len - 1]
        };
        let b = traps[idx - row_len];
        let c = if idx.rem_euclid(row_len) == row_len - 1 {
            false
        } else {
            traps[idx - row_len + 1]
        };

        traps.push(match (a, b, c) {
            (true, true, false) => true,
            (false, true, true) => true,
            (true, false, false) => true,
            (false, false, true) => true,
            _ => false,
        });
    }
    // println!();
    // _print_traps(&traps, &row_len);

    Ok(traps.iter().filter(|&&trap| !trap).count())
}

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
            let a = if idx == 0 {
                false
            } else {
                previos_row[idx - 1]
            };
            let b = previos_row[idx];
            let c = if idx == row_len - 1 {
                false
            } else {
                previos_row[idx + 1]
            };
            current_row[idx] = match (a, b, c) {
                (true, true, false)
                | (false, true, true)
                | (true, false, false)
                | (false, false, true) => true,
                _ => {
                    count += 1;
                    false
                }
            };
        }
        // if idx == 0 {
        //     std::mem::swap(&mut previos_row, &mut current_row);
        // }
        // println!(
        //     "{}   {count}",
        //     String::from_utf8_lossy(&previos_row).as_ref()
        // );
    }

    Ok(count)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, 400_000)
    // let row0 = fs::read_to_string(input_path)?.trim().to_string();
    // let rows = 400_000;

    // // let row0 = ".^^.^.^^^^";
    // let row_len = row0.len();
    // let mut traps: Vec<bool> = row0.bytes().map(|c| c == b'^').collect();

    // for idx in row_len..row_len * rows {
    //     let a = if idx.rem_euclid(row_len) == 0 {
    //         false
    //     } else {
    //         traps[idx - row_len - 1]
    //     };
    //     let b = traps[idx - row_len];
    //     let c = if idx.rem_euclid(row_len) == row_len - 1 {
    //         false
    //     } else {
    //         traps[idx - row_len + 1]
    //     };

    //     traps.push(match (a, b, c) {
    //         (true, true, false) => true,
    //         (false, true, true) => true,
    //         (true, false, false) => true,
    //         (false, false, true) => true,
    //         _ => false,
    //     });
    // }
    // // println!();
    // // _print_traps(&traps, &row_len);
    // //
    // // println!("Size: {}", traps.len() * std::mem::size_of::<bool>());

    // Ok(traps.iter().filter(|&&trap| !trap).count())
}
