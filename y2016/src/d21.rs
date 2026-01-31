use std::fs;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;
use std::path::PathBuf;

#[inline]
fn swap_position(password: &mut [u8], x: usize, y: usize) {}

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let password = &mut b"abcdefgh".to_owned();

    let swap = &mut password.clone();
    println!();
    println!("  01234567");
    println!("_ {}", String::from_utf8_lossy(password));
    for line in reader.lines().flatten() {
        let mut parts = line.split(' ');

        match (parts.next(), parts.next()) {
            (Some("swap"), Some("position")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|x| x.parse::<usize>().ok()),
                    parts.nth(2).and_then(|y| y.parse::<usize>().ok()),
                ) {
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
                    password.swap(x, y);
                }
            }
            (Some("reverse"), Some("positions")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|x| x.parse::<usize>().ok()),
                    parts.nth(1).and_then(|y| y.parse::<usize>().ok()),
                ) {
                    for i in 0..y.strict_sub(x).strict_add(1).div_euclid(2) {
                        password.swap(x.strict_add(i), y.strict_sub(i));
                    }
                }
            }
            (Some("move"), Some("position")) => {
                if let (Some(x), Some(y)) = (
                    parts.next().and_then(|x| x.parse::<usize>().ok()),
                    parts.nth(2).and_then(|y| y.parse::<usize>().ok()),
                ) {
                    if x < y {
                        let s = password[x];
                        for i in x..y {
                            password[i] = password[i.strict_add(1)];
                        }
                        password[y] = s;
                    } else if y < x {
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
                    for i in 0..password.len() {
                        swap[i] = password[i.strict_add(x).rem_euclid(password.len())];
                    }
                    std::mem::swap(password, swap);
                }
            }
            (Some("rotate"), Some("right")) => {
                if let Some(x) = parts.next().and_then(|x| x.parse::<usize>().ok()) {
                    for i in 0..password.len() {
                        swap[i.strict_add(x).rem_euclid(password.len())] = password[i];
                    }
                    std::mem::swap(password, swap);
                }
            }
            (Some("rotate"), Some("based")) => {
                if let Some(x) = parts.nth(4).and_then(|c| c.bytes().next()) {
                    let mut x = password.iter().position(|&c| c == x).unwrap();
                    if x >= 4 {
                        x.add_assign(1);
                    }
                    x.add_assign(1);
                    for i in 0..password.len() {
                        swap[i.strict_add(x).rem_euclid(password.len())] = password[i];
                    }
                    std::mem::swap(password, swap);
                }
            }
            _ => panic!("unexpected: {line}"),
        }
    }
    Ok(String::from_utf8_lossy(password).to_string())
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    // let reader = BufReader::new(fs::File::open(input_path)?);
    let password = &mut b"fbgdceah".to_owned();

    let content = fs::read_to_string(input_path)?;
    let lines: Vec<_> = content.lines().rev().collect();
    // let reader = BufReader::new(fs::File::open("input/y2016/d21/test.txt")?);
    // let password = &mut b"abcde".to_owned();
    // let reader = BufReader::new(fs::File::open("input/y2016/d21/test2.txt")?);

    let swap = &mut password.clone();
    println!();

    println!("  01234567");
    println!("_ {}", String::from_utf8_lossy(password));
    for line in lines {
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
                } else {
                    panic!("!");
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
                    for i in 0..y.strict_sub(x).strict_add(1).div_euclid(2) {
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
                    } else if y < x {
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
                        swap[i] = password[i.strict_add(x).rem_euclid(password.len())];
                    }
                    std::mem::swap(password, swap);
                } else {
                    panic!("!");
                }
            }
            (Some("rotate"), Some("right")) => {
                if let Some(x) = parts.next().and_then(|x| x.parse::<usize>().ok()) {
                    println!("ROTATE RIGHT {x}");
                    for i in 0..password.len() {
                        swap[i.strict_add(x).rem_euclid(password.len())] = password[i];
                    }
                    std::mem::swap(password, swap);
                } else {
                    panic!("!");
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
                        swap[i.strict_add(x).rem_euclid(password.len())] = password[i];
                    }
                    std::mem::swap(password, swap);
                } else {
                    panic!("!");
                }
            }
            _ => panic!("unexpected: {line}"),
        }

        println!("  01234567");
        println!(". {}", String::from_utf8_lossy(password));
    }

    Ok(String::from_utf8_lossy(password).to_string())
}
