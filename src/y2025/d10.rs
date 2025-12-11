use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::{BitOr, BitXor};
use std::path::PathBuf;
use std::{fs, usize};

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;

    let mut index = 0;

    let mut queue: BinaryHeap<(Reverse<u64>, u16)> = BinaryHeap::with_capacity(100);
    let mut visited: HashSet<u16> = HashSet::with_capacity(1000);
    let mut sum: u64 = 0;

    loop {
        let mut state: u16 = 0;
        let mut state_index = 0;
        let mut buttons: Vec<u16> = Vec::new();
        let mut joltages: Vec<u16> = Vec::new();
        let mut joltage = 0;

        loop {
            state = match content[index] {
                b'[' => {
                    state_index = index + 1;
                    state
                }
                b'.' => state,
                b'#' => 1_u16
                    .rotate_left((index - state_index) as u32)
                    .bitxor(state),
                b']' => break,
                c => {
                    let c = c as char;
                    panic!("unexpected char {c:?} at {index}");
                }
            };
            index += 1;
        }
        'buttons: loop {
            index += 1;
            let mut button: u16 = 0;
            loop {
                let x = content[index] as char;
                // print!("{x}");
                button = match content[index] {
                    b'(' | b' ' | b',' => button,
                    b')' => break,
                    b'{' => break 'buttons,
                    n => 1_u16
                        .rotate_left((n.wrapping_sub(b'0')) as u32)
                        .bitxor(button),
                };
                index += 1;
            }
            buttons.push(button);
        }

        loop {
            index += 1;
            joltage = match content[index] {
                b'{' | b' ' => joltage,
                b'}' => {
                    joltages.push(joltage);
                    index += 1;
                    break;
                }
                b',' => {
                    joltages.push(joltage);
                    0
                }
                n => joltage * 10 + u16::from(n.wrapping_sub(b'0')),
                _ => panic!(),
            };
        }
        // print!(". {state:#010b}");

        // for button in &buttons {
        //     print!(" | {button:#010b}");
        // }

        // print!(" {joltages:?}");

        // println!();

        queue.clear();

        queue.push((Reverse(0), 0));

        visited.clear();

        'check: while let Some((Reverse(count), current_state)) = queue.pop() {
            let count = count + 1;
            for button in &buttons {
                let new_state = current_state.bitxor(button);
                if visited.contains(&new_state) {
                    continue;
                }
                if new_state == state {
                    sum += count;
                    break 'check;
                }
                queue.push((Reverse(count), new_state));
                visited.insert(new_state);
            }
        }

        loop {
            if index >= content.len() {
                return Ok(sum);
            }
            match content[index] {
                b' ' | b'\n' => {}
                _ => break,
            };
            index += 1;
        }
    }
    unreachable!();
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;

    let mut index = 0;

    let mut queue: BinaryHeap<(Reverse<u64>, [u16; 10])> = BinaryHeap::with_capacity(100);
    let mut visited: HashSet<[u16; 10]> = HashSet::with_capacity(1000);
    let mut sum: u64 = 0;

    loop {
        let mut state: u16 = 0;
        let mut state_index = 0;
        let mut buttons: Vec<[u16; 10]> = Vec::new();
        let mut joltages: [u16; 10] = [0; 10];
        let mut joltage_index: usize = 0;

        loop {
            state = match content[index] {
                b'[' => {
                    state_index = index + 1;
                    state
                }
                b'.' => state,
                b'#' => 1_u16
                    .rotate_left((index - state_index) as u32)
                    .bitxor(state),
                b']' => break,
                c => {
                    let c = c as char;
                    panic!("unexpected char {c:?} at {index}");
                }
            };
            index += 1;
        }
        'buttons: loop {
            index += 1;
            let mut button: [u16; 10] = [0; 10];
            loop {
                let x = content[index] as char;
                // print!("{x}");
                match content[index] {
                    b'(' | b' ' | b',' => (),
                    b')' => break,
                    b'{' => break 'buttons,
                    n => button[n.wrapping_sub(b'0') as usize] = 1,
                };
                index += 1;
            }
            buttons.push(button);
        }

        loop {
            index += 1;
            match content[index] {
                b'{' | b' ' => {}
                b'}' => {
                    index += 1;
                    break;
                }
                b',' => joltage_index += 1,
                n => {
                    joltages[joltage_index] =
                        joltages[joltage_index] * 10 + u16::from(n.wrapping_sub(b'0'))
                }
                _ => panic!(),
            };
        }
        print!(". {state:#010b}");

        for button in &buttons {
            print!(" | {button:?}");
        }

        print!(" {joltages:?}");

        println!();

        queue.clear();

        queue.push((Reverse(0), [0; 10]));

        visited.clear();

        'check: while let Some((Reverse(count), current_state)) = queue.pop() {
            let count = count + 1;
            for button in &buttons {
                let new_state = std::array::from_fn(|i| current_state[i] + button[i]);
                if visited.contains(&new_state) {
                    continue;
                }
                visited.insert(new_state);

                if new_state.iter().zip(joltages).any(|(a, b)| a > &b) {
                    continue;
                }
                if new_state == joltages {
                    sum += count;
                    break 'check;
                }
                // println!("PUSH {new_state:?} {count} (expected: {joltages:?})");
                queue.push((Reverse(count), new_state));
            }
        }

        loop {
            if index >= content.len() {
                return Ok(sum);
            }
            match content[index] {
                b' ' | b'\n' => {}
                _ => break,
            };
            index += 1;
        }
    }
    unreachable!();
    Ok(0)
}
