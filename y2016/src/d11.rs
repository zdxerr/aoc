use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;

fn valid(floors: &Vec<u16>) -> bool {
    for floor in floors {
        let generators = floor & 0xFF;
        let microchips = floor >> 16;
        let difference = generators ^ microchips;
        if difference > 0 && microchips & difference > 0 && generators & difference > 0 {
            return false;
        }
    }
    true
}

fn _print_state(step: usize, floor: usize, floors: Vec<u16>) {
    println!("##### {step} {floor}");
    for (index, f) in floors.iter().enumerate().rev() {
        println!(
            "{index} {} {f:016b}",
            if floor == index { '#' } else { ' ' }
        );
    }
}

pub fn solve(
    input_path: &PathBuf,
    added_elements: &[&str],
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let content = fs::read_to_string(r"input/y2016/d11/test.txt")?;

    let mut elements = HashMap::new();

    let mut init_floors: Vec<u16> = content
        .lines()
        .map(|line| {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            splitted.windows(2).fold(0, |floor, words| {
                match *&words[1].trim_end_matches(['.', ',']) {
                    "generator" => {
                        let len = elements.len();
                        let element = elements.entry(words[0]).or_insert_with(|| 1 << len);
                        floor | *element
                    }
                    "microchip" => {
                        let len = elements.len();
                        let element = elements
                            .entry(words[0].trim_end_matches("-compatible"))
                            .or_insert_with(|| 1 << len);
                        floor | *element << 8
                    }
                    _ => floor,
                }
            })
        })
        .collect();

    for element in added_elements {
        let len = elements.len();
        let element = elements.entry(element).or_insert_with(|| 1 << len);
        init_floors[0] |= *element << 8;
    }

    // println!();
    // for (key, value) in &elements {
    //     println!("{} .. {:016b}", key, value);
    // }
    // println!("##########");
    // for (index, floor) in init_floors.iter().enumerate().rev() {
    //     println!("{index} {floor:016b}");
    // }

    let mut queue: VecDeque<(usize, usize, Vec<u16>)> = VecDeque::with_capacity(1000);
    let mut visited: HashSet<(usize, Vec<u16>)> = HashSet::with_capacity(1000);

    queue.push_back((0, 0, init_floors));
    while let Some((step, floor, floors)) = queue.pop_front() {
        if !visited.insert((floor, floors.clone())) {
            continue;
        }
        if !valid(&floors) {
            continue;
        }
        if floors[0..floors.len() - 1].iter().all(|&floor| floor == 0) {
            return Ok(step);
        }

        let current_floor = &floors[floor];
        for microchip_index in 0..elements.len() {
            let microchip = 1 << microchip_index;
            if !(current_floor & (microchip << 8) > 0) {
                continue;
            }

            let next_floor = floor + 1;
            if (0..floors.len()).contains(&next_floor) {
                let mut next_floors = floors.clone();
                next_floors[floor] ^= microchip << 8;
                next_floors[next_floor] ^= microchip << 8;

                for microchip_index1 in microchip_index + 1..elements.len() {
                    let microchip1 = 1 << microchip_index1;
                    if !(current_floor & (microchip1 << 8) > 0) {
                        continue;
                    }

                    let mut next_floors = next_floors.clone();
                    next_floors[floor] ^= microchip1 << 8;
                    next_floors[next_floor] ^= microchip1 << 8;
                    queue.push_back((step + 1, next_floor, next_floors));
                }

                if current_floor & microchip > 0 {
                    next_floors[floor] ^= microchip;
                    next_floors[next_floor] ^= microchip;
                }
                queue.push_back((step + 1, next_floor, next_floors));
            }
            let next_floor = floor - 1;
            if (0..floors.len()).contains(&next_floor) {
                let mut next_floors = floors.clone();
                next_floors[floor] ^= microchip << 8;
                next_floors[next_floor] ^= microchip << 8;
                queue.push_back((step + 1, next_floor, next_floors));
            }
        }
    }

    Err("not soultion found".into())
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, &[])
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, &["elerium", "dilithium"])
}
