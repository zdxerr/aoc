mod y2015;
mod y2016;
mod y2023;
mod y2024;
mod y2025;

use std::env;
use std::time::Instant;

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
        rgb_print!(224, 222, 244, "{:40}", $name);

        let input_path = env::current_dir()?
            .join("input")
            .join(year)
            .join(day)
            .join("input.txt");
        let t0 = Instant::now();
        let result = $year::$day::part1(&input_path);
        let duration = t0.elapsed().as_secs_f64() * 1000.0;
        rgb_print!(144, 140, 170, "   1 ");
        rgb_print!(246, 193, 119, "{duration:8.3}ms ⏱ ");
        match result {
            Ok(result) => rgb_print!(196, 167, 231, " {result:>20}"),
            Err(msg) => rgb_print!(235, 111, 146, " {msg:>20}"),
        }
        let t0 = Instant::now();
        let result = $year::$day::part2(&input_path);
        let duration = t0.elapsed().as_secs_f64() * 1000.0;
        rgb_print!(144, 140, 170, "   2 ");
        rgb_print!(246, 193, 119, "{duration:8.3}ms ⏱ ");
        match result {
            Ok(result) => rgb_print!(196, 167, 231, " {result:>20}"),
            Err(msg) => rgb_print!(235, 111, 146, " {msg:>20}"),
        }
        println!();
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t0 = Instant::now();
    // run!(y2015, d01, "Not Quite Lisp");
    // run!(y2015, d02, "I Was Told There Would Be No Math");
    // run!(y2015, d03, "Perfectly Spherical Houses in a Vacuum");
    // run!(y2015, d04, "The Ideal Stocking Stuffer");
    // run!(y2015, d05, "Doesn't He Have Intern-Elves For This?");
    // run!(y2015, d06, "Probably a Fire Hazard");
    // run!(y2015, d07, "Some Assembly Required");
    // run!(y2015, d08, "Matchsticks");
    // run!(y2015, d09, "All in a Single Night");
    // run!(y2015, d10, "Elves Look, Elves Say");
    // run!(y2015, d11, "Corporate Policy");
    // run!(y2015, d12, "JSAbacusFramework.io");
    // run!(y2015, d13, "Knights of the Dinner Table");
    // run!(y2015, d14, "Reindeer Olympics");
    // run!(y2015, d15, "Science for Hungry People");
    // run!(y2015, d16, "Aunt Sue");
    // run!(y2015, d17, "No Such Thing as Too Much");
    // run!(y2015, d18, "Like a GIF For Your Yard");
    // run!(y2015, d19, "Medicine for Rudolph");
    // run!(y2015, d20, "Infinite Elves and Infinite Houses");
    // run!(y2015, d21, "RPG Simulator 20XX");
    // run!(y2015, d22, "Wizard Simulator 20XX");
    // run!(y2015, d23, "Opening the Turing Lock");
    run!(y2015, d24, "It Hangs in the Balance");
    // run!(y2015, d25, "Let It Snow");

    run!(y2016, d01, "No Time for a Taxicab");
    run!(y2016, d02, "Bathroom Security");
    run!(y2016, d04, "Security Through Obscurity");
    // run!(y2023, d02, "Cube Conundrum");

    // run!(y2024, d01, "");
    // run!(y2024, d02, "");
    // run!(y2024, d03, "");
    // run!(y2024, d04, "");
    // run!(y2024, d05, "");
    // run!(y2024, d06, "");
    // run!(y2024, d07, "");
    // run!(y2024, d08, "");
    // run!(y2024, d09, "");
    // run!(y2024, d10, "");
    // run!(y2024, d11, "");
    // run!(y2024, d12, "");
    // run!(y2024, d13, "");
    // run!(y2024, d14, "");
    // run!(y2024, d15, "");
    // run!(y2024, d16, "");
    // run!(y2024, d17, "");
    // run!(y2024, d18, "");
    // run!(y2024, d19, "");
    // run!(y2024, d20, "");
    // run!(y2024, d21, "");
    // run!(y2024, d22, "");
    // run!(y2024, d23, "");
    // run!(y2024, d24, "");
    // run!(y2024, d25, "");

    // run!(y2025, d01, "Secret Entrance");
    // run!(y2025, d02, "Gift Shop");
    // run!(y2025, d03, "Lobby");
    // run!(y2025, d04, "Printing Department");
    // run!(y2025, d05, "Cafeteria");
    // run!(y2025, d06, "Trash Compactor");
    // run!(y2025, d07, "Laboratories");
    // run!(y2025, d08, "Playground");
    // run!(y2025, d09, "Movie Theater");
    // run!(y2025, d10, "Factory");
    // run!(y2025, d11, "Reactor");
    // run!(y2025, d12, "Christmas Tree Farm");
    let duration = t0.elapsed().as_secs_f64() * 1000.0;
    rgb_print!(156, 207, 216, "Total");
    println!();
    rgb_print!(246, 193, 119, "   {duration:8.3}ms ⏱ ");
    println!();

    Ok(())
}
