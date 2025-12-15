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

macro_rules! run {
    ($year:ident, $day:ident, $name:literal) => {{
        let year = stringify!($year);
        let day = stringify!($day);
        rgb_print!(
            156,
            207,
            216,
            "{}.{:02} ",
            year.strip_prefix("y").unwrap(),
            day.strip_prefix("d").unwrap(),
        );
        rgb_print!(224, 222, 244, "{}", $name);
        println!();

        let input_path = env::current_dir()?
            .join("input")
            .join(year)
            .join(day)
            .join("input.txt");
        let t0 = Instant::now();
        let result = $year::$day::part1(&input_path);
        let duration = t0.elapsed().as_micros();
        rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
        rgb_print!(196, 167, 231, "{result:?}");
        println!();
        let t0 = Instant::now();
        let result = $year::$day::part2(&input_path);
        let duration = t0.elapsed().as_micros();
        rgb_print!(246, 193, 119, "⏱ {duration:9}us ");
        rgb_print!(196, 167, 231, "{result:?}");
        println!();
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run!(y2023, d02, "Cube Conundrum");
    //
    let t0 = Instant::now();

    run!(y2025, d01, "Secret Entrance");
    run!(y2025, d02, "Gift Shop");
    run!(y2025, d03, "Lobby");
    run!(y2025, d04, "Printing Department");
    run!(y2025, d05, "Cafeteria");
    run!(y2025, d06, "Trash Compactor");
    run!(y2025, d07, "Laboratories");
    run!(y2025, d08, "Playground");
    run!(y2025, d09, "Movie Theater");
    run!(y2025, d10, "Factory");
    run!(y2025, d11, "Reactor");
    run!(y2025, d12, "Christmas Tree Farm");
    let duration = t0.elapsed().as_micros();
    rgb_print!(246, 193, 119, "## ⏱ {duration:9}us ");
    println!();

    Ok(())
}
