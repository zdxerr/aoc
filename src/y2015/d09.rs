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
        current: &'a str,
        path: &mut HashSet<&'a str>,
        distance: u64,
        distances: &HashMap<&'a str, HashMap<&'a str, u64>>,
    ) -> u64 {
        path.insert(current);
        let new_distance = if path.len() == distances.len() {
            distance
        } else {
            distances[current]
                .iter()
                .filter_map(|(next, next_distance)| {
                    if path.contains(next) {
                        None
                    } else {
                        Some(walk(next, path, distance + next_distance, distances))
                    }
                })
                .min()
                .unwrap()
        };
        path.remove(current);
        new_distance
    }

    Ok(distances
        .keys()
        .map(|start| walk(start, &mut path, 0, &distances))
        .min()
        .unwrap())
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
