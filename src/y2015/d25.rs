use std::fs;
use std::ops::{Mul, RemAssign};
use std::path::PathBuf;

// A simple Linear Congruential Generator (LCG)
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    // Seed with a value (you can use time or any starting number)
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    fn next_usize(&mut self) -> usize {
        // Constants from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state as usize
    }

    fn next_u64(&mut self) -> u64 {
        // Constants from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    fn gen_range(&mut self, max: u32) -> u32 {
        ((self.next_u64() >> 32) as u32) % max
    }

    // Random i32 in range [min, max] inclusive
    fn gen_range_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min + 1) as u64;
        min + (self.next_u64() % range) as i32
    }

    // Random f64 in [0.0, 1.0)
    fn gen_f64(&mut self) -> f64 {
        let val = (self.next_u64() >> 11) as f64; // Take top bits for better distribution
        val / (1u64 << 53) as f64 // 53-bit precision
    }
}

fn random_test_string(length: usize) -> String {
    static WORD_CHAR: [char; 83] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ', '.',
        ',', '!', '?', ';', ':', '\'', '"', '(', ')', '[', ']', '{', '}', '<', '>', '-', '_', '+',
        '/', '\\', '|', '*', '&', '^', '%', '$', '#', '@', '\n',
    ];

    // Seed from current time (nanoseconds)
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut rng = SimpleRng::new(seed);
    (0..length)
        .map(|_| match rng.gen_range(20) {
            0 => format!("{:.5}", rng.gen_f64()),
            1..4 => format!("{}", rng.next_u64()),
            4..7 => format!("{}", rng.gen_range_i32(-6666, 6666)),
            _ => format!(
                "{}",
                WORD_CHAR[rng.next_usize().rem_euclid(WORD_CHAR.len())]
            ),
        })
        .collect()
}

fn pos(row: u64, column: u64) -> u64 {
    // codes are positioned in diagonal order in the table
    // calculate the row and column of the start of this diagonal line of positions
    let row0 = row + column - 1;
    // use Gauss sum to calculate the position of row0, column0 since each diagonal line contains <row> elements
    let pos0 = ((row0 - 1) * (row0 - 1 + 1)) / 2;
    pos0 + column
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    println!("{content}");
    let mut numbers = vec![];
    let mut num = None;
    for c in content.bytes() {
        if c.is_ascii_digit() {
            match num {
                Some(ref mut num) => *num = *num * 10 + (c - b'0') as u64,
                None => num = Some((c - b'0') as u64),
            }
        } else if let Some(number) = num {
            numbers.push(number);
            num = None;
        }
    }

    let [row, column] = numbers
        .try_into()
        .expect("expected row and column not in input");

    // let (row, column) = (2, 4);

    // let (r, c) = (row + column - 1, 1);

    // // Gauss sum
    // let i = ((r - 1) * (r - 1 + 1)) / 2;

    // let p = 1 + i + column - 1;
    // dbg!(&row, &column, &r, &c, &i, &p);

    // println!("{}", pos(1, 6));
    // println!("{}", pos(5, 1));
    // println!("{}", pos(2, 2));

    let pos = pos(row, column);

    // Modular exponentiation: https://en.wikipedia.org/wiki/Modular_exponentiation
    // (a * b) mod m = [(a mod m) * (b mod m)] mod m
    let n = 20151125_u64
        .mul(252533_u64.pow((pos - 1) as u32))
        .rem_euclid(33554393);

    let mut c = 20151125_u64;
    for _ in 0..(pos - 1) {
        c = (c * 252533_u64) % 33554393;
    }

    let mut first = 20151125_u64;
    for i in 2..=pos {
        first = first.mul(252533_u64).rem_euclid(33554393);
        // println!("{i}, {}", first);
    }
    println!("{n}");
    println!("{c}");
    println!("{first}");

    // println!("{c}")
    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
