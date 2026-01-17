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

// fn _print_state(state: &State<&str>) {
//     println!(
//         "{:_^30}",
//         format!(
//             " {} {} ",
//             state.0,
//             if valid(&state.2) { "valid" } else { "INVALID" }
//         )
//     );
//     for (floor_index, (generators, microchips)) in state.2.iter().enumerate().rev() {
//         print!(
//             "{floor_index:01} {:1} ",
//             if floor_index == state.1 { '#' } else { ' ' }
//         );
//         for microchip in microchips {
//             print!("M[{microchip}] ");
//         }
//         for generator in generators {
//             print!("G{{{generator}}} ");
//         }
//         println!();
//     }
// }

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let content = fs::read_to_string(r"input/y2016/d11/test.txt")?;

    let mut elements = HashMap::new();

    let init_floors: Vec<u16> = content
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
        // println!("##### {step} {floor}");
        // for (index, f) in floors.iter().enumerate().rev() {
        //     println!(
        //         "{index} {} {f:016b}",
        //         if floor == index { '#' } else { ' ' }
        //     );
        // }
        // println!();
        // if step > 3 {
        //     continue;
        // }
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

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // You step into the cleanroom separating the lobby from the isolated area and put on the hazmat suit.

    // Upon entering the isolated containment area, however, you notice some extra parts on the first floor that weren't listed on the record outside:

    // An elerium generator.
    // An elerium-compatible microchip.
    // A dilithium generator.
    // A dilithium-compatible microchip.
    // These work just like the other generators and microchips. You'll have to get them up to assembly as well.

    // What is the minimum number of steps required to bring all of the objects, including these four new ones, to the fourth floor?
    Err("not implemented".into())
}
