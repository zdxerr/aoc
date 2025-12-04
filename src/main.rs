mod y2023;
mod y2025;

use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::{env, fs};
// use y2023::d021::part1;

use std::path::PathBuf;

// #[cfg(windows)]
// {
//     use windows::Win32::System::Console::*;
//     unsafe {
//         let handle = GetStdHandle(STD_OUTPUT_HANDLE);
//         let mut mode = 0;
//         GetConsoleMode(handle, &mut mode);
//         SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
//     }
// }

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr $(, bg)?) => {{
        const ESC: &str = "\x1b[";
        const RESET: &str = "\x1b[0m";
        $(
            format!("{}48;2;{};{};{}m", ESC, $r, $g, $b)
        )?
        format!("{}38;2;{};{};{}m", ESC, $r, $g, $b)
    }};
}
macro_rules! rgb_print {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", $r, $g, $b, format!($($arg)*))
    };
}
// HSV → RGB conversion
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let year = 2023;
    let day = 2;
    let title = "Cube Conundrum";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();

    let input_path = env::current_dir()?
        .join("input")
        .join("y2023")
        .join("d02")
        .join("input.txt");
    println!("  Path: {}", input_path.display());
    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(input_path)?);

    let res = y2023::d021::part1(buff_reader);
    let duration = t0.elapsed().as_nanos();
    print!(" ⏱ {duration:12}ns");
    if let Some(res) = res {
        println!(" {res} ❌✔ \u{274C} \u{2714}");
    } else {
        println!(" ∅");
    }
    let year = 2025;
    let day = 1;
    let title = "Secret Entrance";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d01")
        .join("input.txt");
    // .join("test.txt");

    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(input_path)?);
    let mut s: i16 = 50;
    let mut count: usize = 0;
    for line in buff_reader.lines() {
        let line = line.expect("unable to read line");
        let c = line.chars().nth(0).expect("unable to read first char");
        let n = &line[1..].parse::<i16>().expect("unable to parse number");
        // println!("{s:#?} {c:#?} {n:#?}");
        s = match c {
            'R' => (s + n).rem_euclid(100),
            'L' => (s - n).rem_euclid(100),
            _ => panic!("unexpected first char"),
        };
        if s == 0 {
            count += 1;
        }
    }
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{count}");
    println!();

    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d01")
        .join("input.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(&input_path)?);
    let mut s: i16 = 50;
    let mut count: usize = 0;
    for line in buff_reader.lines() {
        let line = line.expect("unable to read line");
        let c = line.chars().nth(0).expect("unable to read first char");
        let n = &line[1..].parse::<i16>().expect("unable to parse number");
        let w = match c {
            'R' => s + n,
            'L' => s - n,
            _ => panic!("unexpected first char"),
        };

        let a = w.div_euclid(100);
        let b = w.rem_euclid(100);
        // print!("{s:#?} {c:#?} {n:#?} {w:#?} {a} {b}");
        if c == 'L' && b == 0 {
            count += 1;
        }
        count += a.abs() as usize; // - 1;
        if c == 'L' && s == 0 {
            count -= 1;
        }
        // println!("  -- {count:?}");
        s = w.rem_euclid(100);
    }

    let duration = t0.elapsed().as_micros();

    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{count}");
    println!();

    let year = 2025;
    let day = 2;
    let title = "Gift Shop";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();

    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d02")
        .join("input.txt");
    // .join("test.txt");

    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(&input_path)?);
    let mut sum: usize = 0;
    for pair in buff_reader.split(b',') {
        let pair = pair?;
        let splitted: Vec<&[u8]> = pair.splitn(2, |v| *v == b'-').collect();
        let a: usize = std::str::from_utf8(splitted[0])?.trim().parse()?;
        let b: usize = std::str::from_utf8(splitted[1])?.trim().parse()?;

        let r = a..=b;
        for id in r {
            let s = id.to_string();
            let p = s.len().midpoint(0);
            let s1 = &s[0..p];
            let s2 = &s[p..];
            let m = s1 == s2;
            if s1 == s2 {
                sum += id;
            }
        }
    }
    let duration = t0.elapsed().as_micros();

    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{sum}");
    println!();

    fn invalid(id: &usize) -> bool {
        let id = id.to_string();
        let len = id.len();

        for chunk_length in 1..=(len / 2) {
            if len % chunk_length != 0 {
                continue;
            }

            let mut invalid = true;

            for chunk_number in 1..(len / chunk_length) {
                let a = &id[0..chunk_length];
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

    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(&input_path)?);
    let mut sum: usize = 0;
    for pair in buff_reader.split(b',') {
        let pair = pair?;
        let splitted: Vec<&[u8]> = pair.splitn(2, |v| *v == b'-').collect();
        let a: usize = std::str::from_utf8(splitted[0])?.trim().parse()?;
        let b: usize = std::str::from_utf8(splitted[1])?.trim().parse()?;

        for id in a..=b {
            if invalid(&id) {
                sum += id;
            }
        }
    }
    let duration = t0.elapsed().as_micros();

    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{sum}");
    println!();

    let year = 2025;
    let day = 3;
    let title = "Lobby";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();

    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d03")
        .join("input.txt");
    // .join("test.txt");

    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(&input_path)?);
    let mut sum: usize = 0;
    for bank in buff_reader.split(b'\n') {
        let bank = bank.unwrap();
        let max = bank[..bank.len() - 1]
            .iter()
            .enumerate()
            .fold(
                (0, 0_u8),
                |max, (ind, &val)| if val > max.1 { (ind, val) } else { max },
            );
        let max2 = bank[max.0 + 1..].iter().max().unwrap();
        sum += str::from_utf8(&[max.1, *max2])?.parse::<usize>().unwrap();
    }

    let duration = t0.elapsed().as_micros();

    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{sum}");
    println!();
    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(&input_path)?);
    let mut sum: usize = 0;
    let mut v: Vec<u8> = Vec::new();
    for bank in buff_reader.split(b'\n') {
        let bank = bank.unwrap();
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

    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{sum}");
    println!();

    let year = 2025;
    let day = 3;
    let title = "Printing Department";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();

    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d04")
        .join("input.txt");
    // .join("test.txt");

    let t0 = Instant::now();
    let mut sum: usize = 0;
    let grid = fs::read(&input_path)?;
    const NL: u8 = b'\n';
    const ROLL: u8 = b'@';
    let row_len = grid.iter().position(|c| c == &NL).unwrap();

    for index in 0..grid.len() {
        if grid[index] != ROLL {
            continue;
        }
        let adjacent_indices = [
            index.checked_sub(row_len + 2),
            index.checked_sub(row_len + 1),
            index.checked_sub(row_len),
            index.checked_sub(1),
            index.checked_add(1),
            index.checked_add(row_len),
            index.checked_add(row_len + 1),
            index.checked_add(row_len + 2),
        ];
        let adjacent_rolls = adjacent_indices
            .iter()
            .flatten()
            .filter(|index| **index < grid.len())
            .filter(|index| grid[**index] == ROLL)
            .count();

        if adjacent_rolls < 4 {
            sum += 1;
        }
    }

    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{sum}");
    println!();

    let t0 = Instant::now();
    let mut sum: usize = 0;
    let mut grid = fs::read(&input_path)?;
    let row_len = grid.iter().position(|c| c == &NL).unwrap();

    loop {
        let last_sum = sum;
        for index in 0..grid.len() {
            if grid[index] != ROLL {
                continue;
            }
            let adjacent_indices = [
                index.checked_sub(row_len + 2),
                index.checked_sub(row_len + 1),
                index.checked_sub(row_len),
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_add(row_len),
                index.checked_add(row_len + 1),
                index.checked_add(row_len + 2),
            ];
            let adjacent_rolls = adjacent_indices
                .iter()
                .flatten()
                .filter(|index| **index < grid.len())
                .filter(|index| grid[**index] == ROLL)
                .count();

            if adjacent_rolls < 4 {
                grid[index] = b'.';
                sum += 1;
            }
        }
        if last_sum == sum {
            break;
        }
    }

    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{sum}");
    println!();
    // for row in buff_reader.split(b'\n') {
    //     println!("{row:?}")
    // }
    // println!("256-color ramp (16–231):");
    // for i in 16..232 {
    //     print!("\x1b[38;5;{}m▓\x1b[0m", i);
    //     if (i - 16) % 36 == 35 {
    //         println!();
    //     }
    // }
    // println!();

    // // 2. Grayscale (232–255)
    // println!("Grayscale:");
    // for i in 232..256 {
    //     print!("\x1b[38;5;{}m▒▒\x1b[0m", i);
    // }
    // println!("\x1b[0m");

    // // 3. True color gradient
    // println!("True color rainbow gradient:");
    // let text = "Rust ❤ True Color!";
    // for (i, c) in text.chars().enumerate() {
    //     let hue = i as f64 * 360.0 / text.len() as f64;
    //     let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
    //     print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
    // }
    // println!();
    Ok(())
}
