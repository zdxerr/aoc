use std::arch::x86_64::_SIDD_NEGATIVE_POLARITY;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let g: Graph = content
        .lines()
        .map(|line| {
            let (device, attached_devices) = line.split_once(':').expect("missing ':' in {line:?}");

            (device, attached_devices.trim().split(' ').collect())
        })
        .collect();

    let mut queue = vec![&"you"];
    let mut count = 0;
    while let Some(device) = queue.pop() {
        if *device == "out" {
            count += 1;
        } else if let Some(attached_devices) = g.get(device) {
            queue.extend(attached_devices);
        }
    }
    Ok(count)
}

enum State {
    UNDEFINED,
    VISITED,
    VALID,
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let g: Graph = content
        .lines()
        .map(|line| {
            let (device, attached_devices) = line.split_once(':').expect("missing ':' in {line:?}");

            (device, attached_devices.trim().split(' ').collect())
        })
        .collect();
    // let mut states: HashMap<&str, usize> = HashMap::with_capacity(g.len());
    // let empty = HashSet::new();
    // let mut queue = vec![(&"svr", HashSet::new())];
    let mut count = 0;

    // fn update_states<'a>(
    //     state: &mut HashMap<&'a str, usize>,
    //     visited: &HashSet<&'a str>,
    //     inc: usize,
    // ) {
    //     visited.iter().for_each(|device| {
    //         state.entry(device).and_modify(|c| *c += 0).or_insert(inc);
    //     });
    // }
    //
    //
    //

    let mut cache: HashMap<&str, (usize, usize, usize)> = HashMap::with_capacity(g.len());
    // number out / dac / fft
    fn compute_paths<'a>(
        from: &'a str,
        g: &'a Graph,
        cache: &mut HashMap<&'a str, (usize, usize, usize)>,
    ) -> (usize, usize, usize) {
        if from == "out" {
            return (1, 0, 0);
        }
        if let Some(result) = cache.get(from) {
            return *result;
        }

        if let Some(attached_devices) = g.get(from) {
            let mut result = attached_devices
                .iter()
                .map(|d| compute_paths(d, g, cache))
                .reduce(|acc, result| (acc.0 + result.0, acc.1 + result.1, acc.2 + result.2))
                .expect("cannot sum recursion");

            if result.0 > 0 {
                if from == "dac" {
                    result.1 += 1;
                } else if from == "fft" {
                    result.2 += 1;
                }
            }

            cache.insert(from, result);
            return result;
        }

        (0, 0, 0)
    }

    let count = compute_paths(&"svr", &g, &mut cache);
    dbg!(&cache);
    dbg!(&count);

    // while let Some((device, mut visited)) = queue.pop() {
    //     println!("{device} found: {visited:?}");
    //       if *device == "out" {
    //         println!("out found: {visited:?}");
    //         if visited.contains(&"fft") && visited.contains(&"dac") {
    //             count += 1;
    //             update_states(&mut states, &visited, 1);
    //             println!("out XXX");
    //         } else {
    //             update_states(&mut states, &visited, 0);
    //         }
    //     } else if visited.contains(device) {
    //         println!("{device} ==> loop ({visited:?})");
    //         // loop
    //     } else if let Some(attached_devices) = g.get(device) {
    //         // println!(" -> {attached_devices:?}");
    //         // let mut visited = visited.clone();
    //         visited.insert(device);
    //         queue.extend(
    //             attached_devices
    //                 .iter()
    //                 .map(|device| (device, visited.clone())),
    //         );
    //     } else {
    //         update_states(&mut states, &visited, 0);
    //         println!("{device} ==> no attache devices");
    //     }
    // }
    Ok(1)
}
