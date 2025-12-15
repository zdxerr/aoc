use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::BitXor;
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
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;

    let mut index = 0;

    let mut queue: BinaryHeap<(Reverse<u64>, [u16; 10])> = BinaryHeap::with_capacity(100);
    let mut visited: HashMap<[u16; 10], usize> = HashMap::with_capacity(1000);
    let mut sum: usize = 0;

    loop {
        let mut state: u16 = 0;
        let mut state_index = 0;
        let mut buttons: Vec<[bool; 10]> = Vec::new();
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
            let mut button: [bool; 10] = [false; 10];
            loop {
                match content[index] {
                    b'(' | b' ' | b',' => (),
                    b')' => break,
                    b'{' => break 'buttons,
                    n => button[n.wrapping_sub(b'0') as usize] = true,
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
            };
        }
        // print!(". {state:#010b}");

        // for button in &buttons {
        //     print!(" | {button:?}");
        // }

        // print!(" {joltages:?}");

        // println!();

        // fn process(joltages: &[u16; 10], buttons: &Vec<[u16; 10]>) -> bool {
        //     let all_even = joltages.iter().all(|&x| x % 2 == 0);

        //     println!(".. {joltages:?} {all_even}");

        //     if all_even {
        //         return true;
        //     }

        //     for button in buttons {
        //         let new_joltages: [u16; 10] =
        //             std::array::from_fn(|i| joltages[i].checked_sub(button[i]).unwrap_or(u16::MAX));
        //         if new_joltages.iter().any(|&x| x == u16::MAX) {
        //             println!("__ overflow");
        //             return false;
        //         y}
        //         if process(&new_joltages, buttons) {
        //             return true;
        //         }
        //     }
        //     false
        // }

        // process(&joltages, &buttons);

        // return Ok(1);
        //
        // to slow, need to ignore the order of button presses....
        //
        fn solve(
            joltages: [u16; 10],
            buttons: &Vec<[bool; 10]>,
            // mut min: usize,
            // visited: &mut HashMap<[u16; 10], usize>,
        ) -> usize {
            let b_len = buttons.len();

            let upper_bound: u16 = joltages.iter().sum();

            // println!(" {buttons:?} - {b_len} / upper bound: {upper_bound}");

            let mut push = vec![0; buttons.len()];

            for (button_index, button) in buttons.iter().enumerate() {
                let push: Vec<u16> = push
                    .iter()
                    .enumerate()
                    .map(|(n, b)| b + ((n == button_index) as u16))
                    .collect();
                // let mut push: Vec<u16> = push
                //     .iter()
                //     .zip(button)
                //     .map(|(p, b)| p + (*b as u16))
                //     .collect();

                // println!(" : {push:?}");
            }
            0
        }

        solve(joltages, &buttons);
        // break;

        // queue.clear();
        // visited.clear();
        // //
        // fn explore(
        //     joltages: [u16; 10],
        //     buttons: &Vec<[u16; 10]>,
        //     mut min: usize,
        //     visited: &mut HashMap<[u16; 10], usize>,
        // ) -> Option<usize> {
        //     let mut queue: BinaryHeap<(Reverse<usize>, [u16; 10])> = BinaryHeap::with_capacity(100);

        //     queue.push((Reverse(0), joltages));

        //     // let mut min: usize = usize::MAX;
        //     while let Some((Reverse(count), joltages)) = queue.pop() {
        //         if count > min {
        //             continue;
        //         }
        //         let all_zero = joltages.iter().all(|&x| x == 0);
        //         let all_even = joltages.iter().all(|&x| x % 2 == 0);

        //         println!(
        //             ".. {count} / {joltages:?}, even: {all_even}, zero: {all_zero}, min: {min}"
        //         );

        //         if all_zero {
        //             return Some(count);
        //         }

        //         if let Some(last_count) = visited.get(&joltages) {
        //             if *last_count <= count {
        //                 continue;
        //             }
        //         };
        //         visited.insert(joltages, count);

        //         if all_even {
        //             let new_joltages: [u16; 10] =
        //                 std::array::from_fn(|i| joltages[i].strict_div(2));
        //             if let Some(new_count) = explore(new_joltages, buttons, min, visited) {
        //                 min = min.min(count + 2 * new_count);
        //             }
        //             // continue;
        //         }

        //         for button in buttons {
        //             let new_joltages: [u16; 10] = std::array::from_fn(|i| {
        //                 joltages[i].checked_sub(button[i]).unwrap_or(u16::MAX)
        //             });
        //             if new_joltages.iter().any(|&x| x == u16::MAX) {
        //                 println!("__ overflow");
        //                 continue;
        //             }
        //             queue.push((Reverse(count + 1), new_joltages));
        //         }
        //     }
        //     Some(min)
        // }

        // if let Some(result) = explore(joltages, &buttons, usize::MAX, &mut visited) {
        //     sum += result;
        //     println!("## {result:?}");
        // } else {
        //     panic!("unexpected result");
        // }

        // visited.clear();

        // // let clicks: Vec<u64> = vec![0, buttons.len()];

        // 'check: while let Some((Reverse(count), current_state)) = queue.pop() {
        //     let count = count + 1;
        //     for button in &buttons {
        //         let new_state = std::array::from_fn(|i| current_state[i] + button[i]);
        //         if visited.contains(&new_state) {
        //             continue;
        //         }
        //         visited.insert(new_state);

        //         if new_state.iter().zip(joltages).any(|(a, b)| a > &b) {
        //             continue;
        //         }
        //         if new_state == joltages {
        //             sum += count;
        //             break 'check;
        //         }
        //         // println!("PUSH {new_state:?} {count} (expected: {joltages:?})");
        //         queue.push((Reverse(count), new_state));
        //     }
        // }
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
}
