mod y2023;

use std::io::{BufRead, BufReader, Error};
use std::time::Instant;
use std::{env, fs};
// use y2023::d021::part1;

use std::path::PathBuf;

fn get_binary_path() -> Option<PathBuf> {
    env::current_exe().ok()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    if let Some(path) = get_binary_path() {
        println!("Running from: {}", path.display());
    } else {
        println!("Could not determine binary path.");
    }

    let cwd: PathBuf = env::current_dir()?;

    println!("Current Working Directory:");
    println!("  Path: {}", cwd.display());
    let config_path = env::current_dir()?
        .join("input")
        .join("y2023")
        .join("d01")
        .join("input.txt");
    println!("  Path: {}", config_path.display());
    let t0 = Instant::now();
    // let buff_reader = BufReader::new(fs::File::open(input_path)?);
    // let res = y2023::d021::part1(buff_reader);
    // println!("{res}");
    Ok(())
}
