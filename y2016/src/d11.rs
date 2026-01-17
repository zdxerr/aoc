use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;

// #[derive(Clone, Default, PartialEq, Eq, Hash)]
// struct Floor<'a> {
//     generators: Vec<&'a str>,
//     microchips: Vec<&'a str>,
// }
type Floors<T> = Vec<(Vec<T>, Vec<T>)>;
type State<T> = (usize, usize, Floors<T>);

fn valid(floors: &Floors<&str>) -> bool {
    for (generators, microchips) in floors {
        let mut generators_active = generators.clone();
        let mut microchips_active = microchips.clone();
        for generator_idx in (0..generators_active.len()).rev() {
            for microchip_idx in (0..microchips_active.len()).rev() {
                if generators_active[generator_idx].eq(microchips_active[microchip_idx]) {
                    generators_active.remove(generator_idx);
                    microchips_active.remove(microchip_idx);
                    break;
                }
            }
        }
        if !generators_active.is_empty() && !microchips_active.is_empty() {
            return false;
        }
    }
    true
}

fn _print_state(state: &State<&str>) {
    println!(
        "{:_^30}",
        format!(
            " {} {} ",
            state.0,
            if valid(&state.2) { "valid" } else { "INVALID" }
        )
    );
    for (floor_index, (generators, microchips)) in state.2.iter().enumerate().rev() {
        print!(
            "{floor_index:01} {:1} ",
            if floor_index == state.1 { '#' } else { ' ' }
        );
        for microchip in microchips {
            print!("M[{microchip}] ");
        }
        for generator in generators {
            print!("G{{{generator}}} ");
        }
        println!();
    }
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let content = fs::read_to_string(r"input/y2016/d11/test.txt")?;

    // let elements = HashMap::new();

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
    let mut visited: HashSet<(usize, Floors<&str>)> = HashSet::new();
    queue.push_back((0, 0, init_floors));

    while let Some(state) = queue.pop_front() {
        if !visited.insert((state.1, state.2.clone())) {
            continue;
        }
        // _print_state(&state);
        if !valid(&state.2) {
            continue;
        }

        let (step, floor, floors) = state;

        if floors[0..floors.len() - 1]
            .iter()
            .map(|(generators, microchips)| generators.is_empty() && microchips.is_empty())
            .all(|c| c)
        {
            return Ok(step);
        }
        let microchips_len = floors[floor].1.len();
        for microchip_index0 in 0..microchips_len {
            let next_floor = floor + 1;
            if (0..floors.len()).contains(&next_floor) {
                let mut next_floors = floors.clone();
                let microchip0 = next_floors[floor].1.remove(microchip_index0);
                next_floors[next_floor].1.push(microchip0);

                if let Some(generator_index) = floors[floor].0.iter().position(|s| s == &microchip0)
                {
                    let generator = next_floors[floor].0.remove(generator_index);
                    next_floors[next_floor].0.push(generator);
                    queue.push_back((step + 1, next_floor, next_floors));
                }

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

    Err("not soultion found".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
