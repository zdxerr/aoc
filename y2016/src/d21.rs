use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;
use std::path::PathBuf;

// swap position 4 with position 0 swaps the first and last letters, producing the input for the next step, ebcda.
// swap letter d with letter b swaps the positions of d and b: edcba.
// reverse positions 0 through 4 causes the entire string to be reversed, producing abcde.
// rotate left 1 step shifts all letters left one position, causing the first letter to wrap to the end of the string: bcdea.
// move position 1 to position 4 removes the letter at position 1 (c), then inserts it at position 4 (the end of the string): bdeac.
// move position 3 to position 0 removes the letter at position 3 (a), then inserts it at position 0 (the front of the string): abdec.
// rotate based on position of letter b finds the index of letter b (1), then rotates the string right once plus a number of times equal to that index (2): ecabd.
// rotate based on position of letter d finds the index of letter d (4), then rotates the string right once, plus a number of times equal to that index, plus an additional time because the index was at least 4, for a total of 6 right rotations: decab.

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let reader = BufReader::new(fs::File::open("input/y2016/d21/test.txt")?);
    // let reader = BufReader::new(fs::File::open("input/y2016/d21/test2.txt")?);

    let mut password = b"abcdefgh".to_owned();
    let password = &mut b"abcde".to_owned();
    let new = &mut b"_____".to_owned();

    for line in reader.lines().flatten() {
        // println!("{line}");
        let mut parts = line.split(' ');

        match (parts.next(), parts.next()) {
            (Some("swap"), Some("position")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|x| x.parse::<usize>().ok()),
                    parts.nth(2).and_then(|y| y.parse::<usize>().ok()),
                ) {
                    println!("{x}->{y}");
                    password.swap(x, y);
                }
            }
            (Some("swap"), Some("letter")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|s| s.bytes().next()),
                    parts.nth(2).and_then(|s| s.bytes().next()),
                ) && let (Some(x), Some(y)) = (
                    password.iter().position(|&c| c == x),
                    password.iter().position(|&c| c == y),
                ) {
                    println!("{x}->{y}");

                    password.swap(x, y);
                }
            }
            (Some("reverse"), Some("positions")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|x| x.parse::<usize>().ok()),
                    parts.nth(1).and_then(|y| y.parse::<usize>().ok()),
                ) {
                    println!("REVERSE {x}->{y}");
                    for i in 0..y.strict_sub(x).div_euclid(2) {
                        password.swap(x.strict_add(i), y.strict_sub(i));
                    }
                }
            }
            (Some("move"), Some("position")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|x| x.parse::<usize>().ok()),
                    parts.nth(2).and_then(|y| y.parse::<usize>().ok()),
                ) {
                    println!("MOVE {x}->{y}");
                    // todo!();
                    if x < y {
                        let s = password[x];
                        for i in x..y {
                            password[i] = password[i.strict_add(1)];
                        }
                        password[y] = s;
                    } else {
                        let s = password[x];
                        for i in (y..x).rev() {
                            password[i.strict_add(1)] = password[i];
                        }
                        password[y] = s;
                    }
                }
            }
            (Some("rotate"), Some("left")) => {
                if let Some(x) = parts.next().and_then(|x| x.parse::<usize>().ok()) {
                    println!("ROTATE LEFT {x}");
                    for i in 0..password.len() {
                        new[i] = password[i.strict_add(x).rem_euclid(password.len())];
                    }
                    std::mem::swap(password, new);
                }
            }
            (Some("rotate"), Some("right")) => {
                if let Some(x) = parts.next().and_then(|x| x.parse::<usize>().ok()) {
                    println!("ROTATE RIGHT {x}");
                    for i in 0..password.len() {
                        new[i.strict_add(x).rem_euclid(password.len())] = password[i];
                    }
                    std::mem::swap(password, new);
                }
            }
            (Some("rotate"), Some("based")) => {
                if let Some(x) = parts.nth(4).and_then(|c| c.bytes().next()) {
                    let mut x = password.iter().position(|&c| c == x).unwrap();
                    if x >= 4 {
                        x.add_assign(1);
                    }
                    x.add_assign(1);
                    println!("ROTATE RIGHT {x}");
                    for i in 0..password.len() {
                        new[i.strict_add(x).rem_euclid(password.len())] = password[i];
                    }
                    std::mem::swap(password, new);
                }
            }
            // rotate based on position of letter b
            _ => panic!("unexpected: {line}"),
        }

        println!(". {}", String::from_utf8_lossy(password));
    }

    Ok(String::from_utf8_lossy(password).to_string())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
