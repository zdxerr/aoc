use std::fs;
use std::path::PathBuf;

const I: u8 = b'i' - b'a';
const O: u8 = b'o' - b'a';
const L: u8 = b'l' - b'a';

fn valid(password: &Vec<u8>) -> bool {
    let mut straight = false;
    let mut doubles = vec![];

    for idx in 0..password.len() {
        if matches!(password[idx], I | O | L) {
            return false;
        }
        if idx < password.len() - 3
            && password[idx + 2] == password[idx + 1] + 1
            && password[idx + 1] == password[idx] + 1
        {
            straight = true;
        }
        if idx > 0 && password[idx - 1] == password[idx] && !doubles.contains(&(idx - 1)) {
            doubles.push(idx);
        }
    }

    straight && doubles.len() >= 2
}

fn next(password: &mut Vec<u8>) {
    for c in password.iter_mut().rev() {
        *c = (*c + 1) % (b'z' - b'a' + 1);
        if *c > 0 {
            return;
        }
    }
}

fn format(password: &Vec<u8>) -> String {
    password.iter().map(|c| char::from(c + b'a')).collect()
}

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut password: Vec<u8> = fs::read(input_path)?
        .trim_ascii()
        .iter()
        .map(|c| c - b'a')
        .collect();

    loop {
        next(&mut password);
        if valid(&password) {
            break;
        }
    }
    Ok(format(&password))
}

pub fn part2(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut password = part1(input_path)?.chars().map(|c| c as u8 - b'a').collect();
    loop {
        next(&mut password);
        if valid(&password) {
            break;
        }
    }
    Ok(format(&password))

    // Err("not implemented".into())
}
