mod y2023;
mod y2025;

use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::{env, fs};

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

// macro_rules! rgb {
//     ($r:expr, $g:expr, $b:expr $(, bg)?) => {{
//         const ESC: &str = "\x1b[";
//         const RESET: &str = "\x1b[0m";
//         $(
//             format!("{}48;2;{};{};{}m", ESC, $r, $g, $b)
//         )?
//         format!("{}38;2;{};{};{}m", ESC, $r, $g, $b)
//     }};
// }
macro_rules! rgb_print {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", $r, $g, $b, format!($($arg)*))
    };
}

// macro_rules! time_it {
//     ($name:expr, $expr:expr) => {{
//         let start = Instant::now();
//         let result = $expr;
//         let duration = start.elapsed();
//         println!("{} took {:.3?}", $name, duration);
//         result
//     }};

//     ($expr:expr) => {{
//         let start = Instant::now();
//         let result = $expr;
//         let duration = start.elapsed();
//         println!("Expression took {:.3?}", duration);
//         result
//     }};
// }
macro_rules! run {
    ($module:item, $name:literal) => {{
        // let start = Instant::now();
        // let result = $expr;
        // let duration = start.elapsed();
        // println!("{} took {:.3?}", $name, duration);
        // result

        let year = $year;
        let day = $day;
        let title = $name;
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
        let result = y2025::d01::part1(&input_path);

        let duration = t0.elapsed().as_micros();
        rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
        rgb_print!(196, 167, 231, "{result:?}");
        println!();
    }};
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
    let result = y2025::d01::part1(&input_path);

    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();

    let t0 = Instant::now();
    let result = y2025::d01::part2(&input_path);

    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
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

        for id in a..=b {
            let len = id.checked_ilog10().unwrap() + 1;
            let p = len.checked_div(2).unwrap();
            // if p == 0 {
            //     continue;
            // }
            let a = id.div_euclid(10_usize.pow(p));
            let b = id.rem_euclid(10_usize.pow(p));
            if a == b {
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
    let day = 4;
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
    let result = y2025::d04::part1(&input_path);

    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result}");
    println!();

    let t0 = Instant::now();
    let sum = y2025::d04::part2(&input_path);

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
    //
    //
    let year = 2025;
    let day = 5;
    let title = "Cafeteria";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();

    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d05")
        .join("input.txt");
    // .join("test.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let result = y2025::d05::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d05::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();

    let year = 2025;
    let day = 6;
    let title = "Trash Compactor";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d06")
        .join("input.txt");
    // .join("test.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let result = y2025::d06::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d06::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();

    let year = 2025;
    let day = 7;
    let title = "Laboratories";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d07")
        .join("input.txt");
    // .join("test.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let result = y2025::d07::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d07::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();

    let year = 2025;
    let day = 8;
    let title = "Playground";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d08")
        .join("input.txt");
    // .join("test.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let result = y2025::d08::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d08::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();

    let year = 2025;
    let day = 9;
    let title = "Movie Theater";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d09")
        .join("input.txt"); // 226029968 too low!
    // .join("test.txt");
    // .join("test2.txt"); // = 30

    let t0 = Instant::now();
    let result = y2025::d09::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d09::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let year = 2025;
    let day = 10;
    let title = "Factory";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d10")
        // .join("input.txt"); // 226029968 too low!
        .join("test.txt");
    // .join("test2.txt"); // = 30

    let t0 = Instant::now();
    let result = y2025::d10::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d10::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let year = 2025;
    let day = 11;
    let title = "Reactor";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d11")
        .join("input.txt");
    // .join("test.txt");
    // .join("test2.txt");

    let t0 = Instant::now();
    let result = y2025::d11::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d11::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let year = 2025;
    let day = 12;
    let title = "Christmas Tree Farm";
    rgb_print!(156, 207, 216, "{year}.{day:02} ");
    rgb_print!(224, 222, 244, "{title:20}");
    println!();
    let input_path = env::current_dir()?
        .join("input")
        .join("y2025")
        .join("d12")
        .join("input.txt");
    let t0 = Instant::now();
    let result = y2025::d12::part1(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();
    let t0 = Instant::now();
    let result = y2025::d12::part2(&input_path);
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
    rgb_print!(196, 167, 231, "{result:?}");
    println!();

    // run!(y2025::d12, "Christmas Tree Farm");
    Ok(())
}
