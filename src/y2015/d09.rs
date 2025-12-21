use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

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

    let mut path: HashSet<&str> = HashSet::with_capacity(8);

    fn walk<'a>(
        left: &'a str,
        right: &'a str,
        path: &mut HashSet<&'a str>,
        distance: u64,
        distances: &HashMap<&'a str, HashMap<&'a str, u64>>,
    ) -> u64 {
        // println!("{left}-{right} [{path:?}] ({distance})");
        // path.insert(current);
        let new_distance = if path.len() == distances.len() {
            distance
        } else {
            let mut new_distance = u64::MAX;
            for key in distances.keys() {
                // println!("{left} / {key}");
                // let a = distances[left][key];
                // let b = distances[right][key];
                //
                if path.contains(key) {
                    continue;
                }

                match (distances[left].get(key), distances[right].get(key)) {
                    (Some(a), Some(b)) => {
                        if a < b {
                            path.insert(key);
                            new_distance =
                                new_distance.min(walk(key, right, path, distance + a, distances));
                            path.remove(key);
                        } else {
                            path.insert(key);
                            new_distance =
                                new_distance.min(walk(left, key, path, distance + b, distances));
                            path.remove(key);
                        }
                    }
                    (Some(a), None) => {
                        path.insert(key);
                        new_distance =
                            new_distance.min(walk(key, right, path, distance + a, distances));
                        path.remove(key);
                    }
                    (None, Some(b)) => {
                        path.insert(key);
                        new_distance =
                            new_distance.min(walk(left, key, path, distance + b, distances));
                        path.remove(key);
                    }
                    _ => (),
                }
            }
            // distances[current]
            //     .iter()
            //     .filter_map(|(next, next_distance)| {
            //         if path.contains(next) {
            //             None
            //         } else {
            //             Some(walk(next, path, distance + next_distance, distances))
            //         }
            //     })
            //     .min()
            //     .unwrap()
            new_distance
        };
        // path.remove(current);
        new_distance
    }
    let start = distances.keys().next().unwrap();
    path.insert(start);
    Ok(walk(start, start, &mut path, 0, &distances))
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

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

    let mut path: HashSet<&str> = HashSet::with_capacity(8);

    let mut bound = u64::MAX;

    fn walk<'a>(
        current: &'a str,
        path: &mut HashSet<&'a str>,
        distance: u64,
        distances: &HashMap<&'a str, HashMap<&'a str, u64>>,
        bound: &mut u64,
    ) -> u64 {
        path.insert(current);
        let new_distance = if path.len() == distances.len() {
            *bound = distance;
            distance
        } else if distance > *bound {
            distance
        } else {
            distances[current]
                .iter()
                .filter_map(|(next, next_distance)| {
                    if path.contains(next) {
                        None
                    } else {
                        Some(walk(next, path, distance + next_distance, distances, bound))
                    }
                })
                .max()
                .unwrap()
        };
        path.remove(current);
        new_distance
    }

    Ok(distances
        .keys()
        .map(|start| walk(start, &mut path, 0, &distances, &mut bound))
        .max()
        .unwrap())
}
