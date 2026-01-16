use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

type Floors<T> = Vec<(Vec<T>, Vec<T>)>;
type State<T> = (usize, usize, Floors<T>);

fn valid(floors: &Floors<&str>) -> bool {
    for (generators, microchips) in floors {
        for microchip in microchips {
            if !generators.is_empty() && !generators.iter().any(|generator| generator.eq(microchip))
            {
                return false;
            }
        }
    }
    // for microchip in state.2[state.1].1 {
    //     for generator in state.2[state.1].0 {
    //         if microchip == generator {
    //             return false;
    //         }
    //     }
    // }
    true
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // let content = fs::read_to_string(input_path)?;
    let content = fs::read_to_string(r"input/y2016/d11/test.txt")?;

    let init_floors: Floors<&str> = content
        .lines()
        .map(|line| {
            let (mut generators, mut microchips) = (Vec::new(), Vec::new());
            let splitted: Vec<&str> = line.split_whitespace().collect();
            splitted
                .windows(2)
                .for_each(|words| match *&words[1].trim_end_matches(['.', ',']) {
                    "generator" => generators.push(words[0]),
                    "microchip" => microchips.push(words[0].trim_end_matches("-compatible")),
                    _ => (),
                });
            (generators, microchips)
        })
        .collect();
    let mut queue: VecDeque<State<&str>> = VecDeque::with_capacity(1000);
    queue.push_back((0, 0, init_floors));

    while let Some((step, floor, floors)) = queue.pop_front() {
        if !valid(&floors) {
            continue;
        }
        // println!();
        // for (floor_index, (generators, microchips)) in floors.iter().enumerate().rev() {
        //     print!(
        //         "{floor_index:01} {:1} ",
        //         if floor_index == floor { '#' } else { ' ' }
        //     );
        //     for microchip in microchips {
        //         print!("[{microchip}] ");
        //     }
        //     for generator in generators {
        //         print!("{{{generator}}} ");
        //     }
        //     println!();
        // }

        if floors[0..floors.len() - 1]
            .iter()
            .map(|(generators, microchips)| generators.is_empty() && microchips.is_empty())
            .all(|c| c)
        {
            println!();
            for (floor_index, (generators, microchips)) in floors.iter().enumerate().rev() {
                print!(
                    "{floor_index:01} {:1} ",
                    if floor_index == floor { '#' } else { ' ' }
                );
                for microchip in microchips {
                    print!("[{microchip}] ");
                }
                for generator in generators {
                    print!("{{{generator}}} ");
                }
                println!();
            }
            return Ok(step);
        }
        let microchips_len = floors[floor].1.len();
        for microchip_index0 in 0..microchips_len {
            let next_floor = floor + 1;
            if (0..floors.len()).contains(&next_floor) {
                let mut next_floors = floors.clone();
                // dbg!(&next_floors);
                // println!("{:?}", next_floors[floor].0);
                let microchip0 = next_floors[floor].1.remove(microchip_index0);
                next_floors[next_floor].1.push(microchip0);

                if let Some(generator_index) =
                    next_floors[floor].0.iter().position(|s| s == &microchip0)
                {
                    let generator = next_floors[floor].0.remove(generator_index);
                    next_floors[next_floor].0.push(generator);
                }

                queue.push_back((step + 1, next_floor, next_floors));

                for microchip_index1 in microchip_index0 + 1..microchips_len {
                    let mut next_floors = floors.clone();
                    let microchip1 = next_floors[floor].1.remove(microchip_index1); // this will change the index of following elements
                    let microchip0 = next_floors[floor].1.remove(microchip_index0);
                    next_floors[next_floor].1.push(microchip1);
                    next_floors[next_floor].1.push(microchip0);

                    queue.push_back((step + 1, next_floor, next_floors));
                }
            }
            let next_floor = floor - 1;
            if (0..floors.len()).contains(&next_floor) {
                let mut next_floors = floors.clone();
                let microchip0 = next_floors[floor].1.remove(microchip_index0);
                next_floors[next_floor].1.push(microchip0);
                queue.push_back((step + 1, next_floor, next_floors));
            }
        }
    }

    println!();

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
