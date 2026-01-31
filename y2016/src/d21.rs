use std::fs;
use std::path::PathBuf;

#[inline]
fn swap_position(password: &mut [u8], x: usize, y: usize) {
    password.swap(x, y);
}

#[inline]
fn reverse_positions(password: &mut [u8], x: usize, y: usize) {
    for i in 0..y.strict_sub(x).strict_add(1).div_euclid(2) {
        password.swap(x.strict_add(i), y.strict_sub(i));
    }
}

#[inline]
fn move_position(password: &mut [u8], x: usize, y: usize) {
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

#[inline]
fn rotate_left(password: &mut [u8], x: usize, swap: &mut [u8]) {
    for i in 0..password.len() {
        swap[i] = password[i.strict_add(x).rem_euclid(password.len())];
    }
}

#[inline]
fn rotate_right(password: &mut [u8], x: usize, swap: &mut [u8]) {
    for i in 0..password.len() {
        swap[i.strict_add(x).rem_euclid(password.len())] = password[i];
    }
}

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let password = &mut b"abcdefgh".to_owned();
    let swap = &mut password.clone();

    let content = fs::read_to_string(input_path)?;
    let lines: Vec<_> = content.lines().collect();

    for line in lines {
        let mut parts = line.split(' ');

        let p = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        );

        if let (Some("swap"), Some("position"), Some(x), Some(y)) = (p.0, p.1, p.2, p.5) {
            swap_position(password, x.parse()?, y.parse()?);
        } else if let (Some("swap"), Some("letter"), Some(a), Some(b)) = (
            p.0,
            p.1,
            p.2.and_then(|s| s.bytes().next()),
            p.5.and_then(|s| s.bytes().next()),
        ) && let (Some(x), Some(y)) = (
            password.iter().position(|&c| c == a),
            password.iter().position(|&c| c == b),
        ) {
            swap_position(password, x, y);
        } else if let (Some("reverse"), Some("positions"), Some(x), Some(y)) = (p.0, p.1, p.2, p.4)
        {
            reverse_positions(password, x.parse()?, y.parse()?);
        } else if let (Some("move"), Some("position"), Some(x), Some(y)) = (p.0, p.1, p.2, p.5) {
            move_position(password, x.parse()?, y.parse()?);
        } else if let (Some("rotate"), Some("left"), Some(x)) = (p.0, p.1, p.2) {
            rotate_left(password, x.parse()?, swap);
            std::mem::swap(password, swap);
        } else if let (Some("rotate"), Some("right"), Some(x)) = (p.0, p.1, p.2) {
            rotate_right(password, x.parse()?, swap);
            std::mem::swap(password, swap);
        } else if let (Some("rotate"), Some("based"), Some(a)) =
            (p.0, p.1, p.6.and_then(|c| c.bytes().next()))
            && let Some(mut x) = password.iter().position(|&c| c == a)
        {
            if x >= 4 {
                x += 1;
            }
            x += 1;
            rotate_right(password, x, swap);
            std::mem::swap(password, swap);
        } else {
            return Err(format!("unexpected: {line}").into());
        }
    }
    Ok(String::from_utf8_lossy(password).to_string())
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let password = &mut b"fbgdceah".to_owned();
    let swap = &mut password.clone();

    let content = fs::read_to_string(input_path)?;
    let lines: Vec<_> = content.lines().rev().collect();

    for line in lines {
        let mut parts = line.split(' ');

        let p = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        );

        if let (Some("swap"), Some("position"), Some(x), Some(y)) = (p.0, p.1, p.2, p.5) {
            swap_position(password, x.parse()?, y.parse()?);
        } else if let (Some("swap"), Some("letter"), Some(a), Some(b)) = (
            p.0,
            p.1,
            p.2.and_then(|s| s.bytes().next()),
            p.5.and_then(|s| s.bytes().next()),
        ) && let (Some(x), Some(y)) = (
            password.iter().position(|&c| c == a),
            password.iter().position(|&c| c == b),
        ) {
            swap_position(password, x, y);
        } else if let (Some("reverse"), Some("positions"), Some(x), Some(y)) = (p.0, p.1, p.2, p.4)
        {
            reverse_positions(password, x.parse()?, y.parse()?);
        } else if let (Some("move"), Some("position"), Some(x), Some(y)) = (p.0, p.1, p.2, p.5) {
            move_position(password, y.parse()?, x.parse()?); // swap in reverse
        } else if let (Some("rotate"), Some("left"), Some(x)) = (p.0, p.1, p.2) {
            rotate_right(password, x.parse()?, swap);
            std::mem::swap(password, swap);
        } else if let (Some("rotate"), Some("right"), Some(x)) = (p.0, p.1, p.2) {
            rotate_left(password, x.parse()?, swap);
            std::mem::swap(password, swap);
        } else if let (Some("rotate"), Some("based"), Some(a)) =
            (p.0, p.1, p.6.and_then(|c| c.bytes().next()))
            && let Some(x) = password.iter().position(|&c| c == a)
        {
            // calculate the expected rotation based on the current position
            let x = match x {
                0 => 1, // 9
                1 => 1,
                2 => 6,
                3 => 2,
                4 => 7,
                5 => 3,
                6 => continue, // 8
                7 => 4,
                _ => panic!("invalid rotation base {x}"),
            };
            rotate_left(password, x, swap);
            std::mem::swap(password, swap);
        } else {
            return Err(format!("unexpected: {line}").into());
        }
    }
    Ok(String::from_utf8_lossy(password).to_string())
}
