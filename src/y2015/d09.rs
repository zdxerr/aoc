use std::collections::{HashMap, HashSet};
use std::ffi::os_str::Display;
use std::fs;
use std::path::PathBuf;

fn parse(content: &str) -> HashMap<&str, HashMap<&str, u64>> {
    let mut distances: HashMap<&str, HashMap<&str, u64>> = HashMap::with_capacity(8);
    content.lines().for_each(|line| {
        let mut parts = line.split(' ');
        let (from, _, to, _, distance) = (
            parts.next().unwrap(),
            parts.next(),
            parts.next().unwrap(),
            parts.next(),
            parts
                .next()
                .unwrap()
                .parse::<u64>()
                .expect("unable to parse distance"),
        );

        distances
            .entry(from)
            .or_default()
            .entry(to)
            .insert_entry(distance);
        distances
            .entry(to)
            .or_default()
            .entry(from)
            .insert_entry(distance);
    });
    distances
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let distances = parse(&content);

    let mut path: HashSet<&str> = HashSet::with_capacity(8);

    fn walk<'a>(
        left: &'a str,
        right: &'a str,
        path: &mut HashSet<&'a str>,
        distance: u64,
        distances: &HashMap<&'a str, HashMap<&'a str, u64>>,
    ) -> u64 {
        let new_distance = if path.len() == distances.len() {
            distance
        } else {
            let mut new_distance = u64::MAX;
            for key in distances.keys() {
                if path.contains(key) {
                    continue;
                }
                path.insert(key);
                new_distance = new_distance.min(
                    match (distances[left].get(key), distances[right].get(key)) {
                        (Some(a), Some(b)) => {
                            if a < b {
                                walk(key, right, path, distance + a, distances)
                            } else {
                                walk(left, key, path, distance + b, distances)
                            }
                        }
                        (Some(a), None) => walk(key, right, path, distance + a, distances),
                        (None, Some(b)) => walk(left, key, path, distance + b, distances),
                        _ => new_distance,
                    },
                );
                path.remove(key);
            }
            new_distance
        };
        new_distance
    }
    let start = distances.keys().next().unwrap();
    path.insert(start);
    Ok(walk(start, start, &mut path, 0, &distances))
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let distances = parse(&content);

    let mut path: HashSet<&str> = HashSet::with_capacity(8);

    fn walk<'a>(
        left: &'a str,
        right: &'a str,
        path: &mut HashSet<&'a str>,
        distance: u64,
        distances: &HashMap<&'a str, HashMap<&'a str, u64>>,
    ) -> u64 {
        let new_distance = if path.len() == distances.len() {
            distance
        } else {
            let mut new_distance = u64::MIN;
            for key in distances.keys() {
                if path.contains(key) {
                    continue;
                }
                path.insert(key);
                new_distance = new_distance.max(
                    match (distances[left].get(key), distances[right].get(key)) {
                        (Some(a), Some(b)) => {
                            if a > b {
                                walk(key, right, path, distance + a, distances)
                            } else {
                                walk(left, key, path, distance + b, distances)
                            }
                        }
                        (Some(a), None) => walk(key, right, path, distance + a, distances),
                        (None, Some(b)) => walk(left, key, path, distance + b, distances),
                        _ => new_distance,
                    },
                );
                path.remove(key);
            }
            new_distance
        };
        new_distance
    }
    let start = distances.keys().next().unwrap();
    path.insert(start);
    Ok(walk(start, start, &mut path, 0, &distances))
}
