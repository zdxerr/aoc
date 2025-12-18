use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

// fn process_file(path: &str) -> io::Result<Vec<i32>> {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);

//     let numbers: Vec<i32> = reader
//         .lines()
//         .map(|res| res?) // Now OK! Because the whole function returns Result<_, io::Error>
//         .filter(|line| !line.trim().starts_with('#'))
//         .filter_map(|line| line.trim().parse::<i32>().ok())
//         .collect();

//     Ok(numbers)
// }

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);

    let count = reader
        .lines()
        .map(|res| res.unwrap())
        .filter(|line| {
            let mut twice = false;
            let mut aeiou: u64 = 0;
            for idx in 0..line.len() {
                match line.get(idx..idx + 2) {
                    Some("ab" | "cd" | "pq" | "xy") => {
                        // println!(" {line} XX");
                        return false;
                    }
                    Some(s) => {
                        twice |= {
                            let mut chars = s.chars();
                            chars.next() == chars.next()
                        }
                    }
                    _ => (),
                }
                aeiou += match line.get(idx..idx + 1) {
                    Some("a" | "e" | "i" | "o" | "u") => 1,
                    _ => 0,
                }
                // aeiou |= match line.get(idx..idx + 1) {
                //     Some("a") => 0b1,
                //     Some("e") => 0b10,
                //     Some("i") => 0b100,
                //     Some("o") => 0b1000,
                //     Some("u") => 0b10000,
                //     _ => 0b0,
                // }
            }
            // let ones = aeiou.count_ones();
            // let x = aeiou.count_ones() >= 3 && twice;
            // println!(" {line} {aeiou:#b} {twice}");
            aeiou >= 3 && twice //too low
            // a + e + i + o + u >= 3
            // true
        })
        .count();
    Ok(count)
}

// --- Part Two ---
// Realizing the error of his ways, Santa has switched to a better model of
// determining whether a string is naughty or nice. None of the old rules
// apply, as they are all clearly ridiculous.

// Now, a nice string is one with all of the following properties:

// It contains a pair of any two letters that appears at least twice in the
// string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not
// like aaa (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter
// between them, like xyx, abcdefeghi (efe), or even aaa.
//
// For example:

// qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and
// a letter that repeats with exactly one letter between them (zxz).
// xxyxx is nice because it has a pair that appears twice and a letter that
// repeats with one between, even though the letters used by each rule overlap.
// uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a
// single letter between them.
// ieodomkazucvgmuy is naughty because it has a repeating letter with one
// between (odo), but no pair that appears twice.
//
// How many strings are nice under these new rules?

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    reader.bytes();
    Err("not implemented".into())
}
