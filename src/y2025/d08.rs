use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fs, usize};
const START: char = 'S';
use std::collections::BinaryHeap;
pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // let content = fs::read(input_path)?;
    let file = File::open(input_path)?;
    let reader = BufReader::with_capacity(1024, file);

    let positions: Vec<(u64, u64, u64)> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut splitted = line
                .splitn(3, ',')
                .map(|number| number.parse::<u64>().unwrap());
            (
                splitted.next().unwrap(),
                splitted.next().unwrap(),
                splitted.next().unwrap(),
            )
        })
        .collect();

    let mut distances: BinaryHeap<(Reverse<u64>, (usize, usize))> =
        BinaryHeap::with_capacity(positions.len().pow(2));

    for a in 0..positions.len() {
        for b in a..positions.len() {
            if a == b {
                continue;
            }
            let (ax, ay, az) = positions[a];
            let (bx, by, bz) = positions[b];

            distances.push((
                Reverse(
                    (ax.abs_diff(bx)).pow(2) + (ay.abs_diff(by)).pow(2) + (az.abs_diff(bz)).pow(2),
                ),
                (a, b),
            ));
        }
    }

    let mut unions: Vec<usize> = vec![usize::MAX; positions.len()];

    for _ in 0..1000 {
        let (distance, (a, b)) = distances.pop().unwrap();

        let posa = positions[a];
        let posb = positions[b];

        // println!("{distance:?} {posa:?} {posb:?}");

        let ua = unions[a];
        let ub = unions[b];

        let parent = match (ua, ub) {
            (usize::MAX, usize::MAX) => a.min(b),
            (n, usize::MAX) | (usize::MAX, n) => n,
            (a, b) => {
                let u = a.min(b);
                let old = a.max(b);
                unions
                    .iter_mut()
                    .filter(|value| **value == old)
                    .for_each(|value| *value = u);
                u
            }
        };

        unions[a] = parent;
        unions[b] = parent;
    }
    // dbg!(&unions);
    let mut unions2: HashMap<&usize, usize> = HashMap::new();
    for u in unions.iter().filter(|n| **n != usize::MAX) {
        unions2
            .entry(u)
            .and_modify(|c| {
                *c += 1;
            })
            .or_insert(1);
    }

    let mut unions2: Vec<&usize> = unions2.values().collect();
    unions2.sort_unstable();
    // dbg!(&unions2);

    let result = unions2
        .into_iter()
        .rev()
        .take(3)
        .fold(1_usize, |a, b| a * b);

    Ok(result)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;

    Ok(0)
}
