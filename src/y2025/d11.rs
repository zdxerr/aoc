use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;
type Cache<'a> = HashMap<(&'a str, bool, bool), usize>;

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

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let g: Graph = content
        .lines()
        .map(|line| {
            let (device, attached_devices) = line.split_once(':').expect("missing ':' in {line:?}");

            (device, attached_devices.trim().split(' ').collect())
        })
        .collect();

    let mut cache: Cache = HashMap::with_capacity(g.len());
    fn compute_paths<'a>(
        from: &'a str,
        g: &'a Graph,
        cache: &mut Cache<'a>,
        dac: bool,
        fft: bool,
    ) -> usize {
        if from == "out" && dac && fft {
            return 1;
        }
        if let Some(result) = cache.get(&(from, dac, fft)) {
            return *result;
        }

        if let Some(attached_devices) = g.get(from) {
            let result = attached_devices
                .iter()
                .map(|d| compute_paths(d, g, cache, dac || from == "dac", fft || from == "fft"))
                .sum();
            cache.insert((from, dac, fft), result);
            return result;
        }

        0
    }

    Ok(compute_paths(&"svr", &g, &mut cache, false, false))
}
