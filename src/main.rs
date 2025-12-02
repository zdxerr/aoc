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
    // println!("Hello, world!");
    // if let Some(path) = get_binary_path() {
    //     println!("Running from: {}", path.display());
    // } else {
    //     println!("Could not determine binary path.");
    // }

    // let cwd: PathBuf = env::current_dir()?;

    // println!("Current Working Directory:");
    // println!("  Path: {}", cwd.display());
    //
    //
    //
    // run!(2023, 2, 1);
    let year = 2023;
    let day = 2;
    let part = 1;
    let title = "Cube Conundrum";
    print!("{year}.{day:02}.{part} {title:20}");

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
        print!(" {res} ❌✔ \u{274C} \u{2714}");
    } else {
        println!(" ∅");
    }

    // rgb_print!()
    println!("2025.01.1 Secret Entrance ");

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
        println!("{s:#?} {c:#?} {n:#?}");
        s = match c {
            'R' => (s + n).rem_euclid(100),
            'L' => (s - n).rem_euclid(100),
            _ => panic!("unexpected first char"),
        };
        // dbg!(line.churs().nth(0).expect("unable to read first char"));
        //
        if s == 0 {
            count += 1;
        }
    }
    let duration = t0.elapsed().as_nanos();
    println!("X ⏱ {duration:12}ns {count}");

    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d01")
        .join("input.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let buff_reader = BufReader::new(fs::File::open(input_path)?);
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
        print!("{s:#?} {c:#?} {n:#?} {w:#?} {a} {b}");
        if c == 'L' && b == 0 {
            count += 1;
        }
        // if a < 0 {
        count += a.abs() as usize; // - 1;
        // } else {
        // count += a.abs() as usize;
        // }
        if c == 'L' && s == 0 {
            count -= 1;
        }
        println!("  -- {count:?}");
        s = w.rem_euclid(100);
    }
    let duration = t0.elapsed().as_nanos();
    println!("X ⏱ {duration:12}ns {count}");
    // 1. 256-color ramp
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
