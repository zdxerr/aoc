use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
const START: char = 'S';

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let mut count = 0;
    let mut lines = content.lines();

    let start = lines.next().unwrap()?.find(START).unwrap();

    let mut rays: HashSet<usize> = HashSet::with_capacity(100);
    rays.insert(start);
    let mut split: HashSet<usize> = HashSet::with_capacity(100);

    for line in lines {
        let line = line?;

        for ray_index in &rays {
            match line.chars().nth(*ray_index) {
                Some('^') => {
                    split.insert(*ray_index);
                    count += 1;
                }
                _ => (),
            }
        }
        for ray_index in &split {
            rays.remove(&ray_index);
            rays.insert(ray_index - 1);
            rays.insert(ray_index + 1);
        }
        split.clear();
    }

    Ok(count)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let mut lines = content.lines();

    let start = lines.next().unwrap()?.find(START).unwrap();
    let mut rays: HashMap<usize, usize> = HashMap::with_capacity(1000);
    rays.insert(start, 1);

    let mut split: HashSet<usize> = HashSet::with_capacity(100);

    for line in lines {
        let line = line?;

        for ray_index in rays.keys() {
            match line.chars().nth(*ray_index) {
                Some('^') => {
                    split.insert(*ray_index);
                }
                _ => (),
            }
        }
        for ray_index in &split {
            let count = rays.remove(ray_index).unwrap();

            let left_index = ray_index.wrapping_sub(1);
            let right_index = ray_index.wrapping_add(1);

            *rays.entry(left_index).or_insert(0) += count;
            *rays.entry(right_index).or_insert(0) += count;
        }
        split.clear();
    }

    Ok(rays.values().sum())
}
