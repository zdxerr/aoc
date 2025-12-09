use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fs, usize};

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let positions: Vec<(u64, u64)> = content
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut splitted = line
                .splitn(2, ',')
                .map(|number| number.parse::<u64>().unwrap());
            (splitted.next().unwrap(), splitted.next().unwrap())
        })
        .collect();

    let len = positions.len();

    let mut largest_area = 0;
    for a in 0..len {
        for b in a + 1..len {
            let (a, b) = (positions[a], positions[b]);
            let x = a.0.abs_diff(b.0) + 1;
            let y = a.1.abs_diff(b.1) + 1;
            let area = x * y;
            largest_area = largest_area.max(area);
        }
    }

    Ok(largest_area)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let positions: Vec<(u64, u64)> = content
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut splitted = line
                .splitn(2, ',')
                .map(|number| number.parse::<u64>().unwrap());
            (splitted.next().unwrap(), splitted.next().unwrap())
        })
        .collect();

    let len = positions.len();

    let mut edges: Vec<((u64, u64), (u64, u64))> =
        positions.windows(2).map(|w| (w[0], w[1])).collect();

    edges.push((positions[positions.len() - 1], positions[0]));
    // dbg!(edges);

    let mut largest_area = 0;
    for a in 0..len {
        for b in a + 1..len {
            println!("{a:?} | {b:?}");
            let (a, b) = (positions[a], positions[b]);

            println!(" . {a:?} | {b:?}");
            for e in &edges {
                println!(" . {e:?}");
            }

            let x = a.0.abs_diff(b.0) + 1;
            let y = a.1.abs_diff(b.1) + 1;
            let area = x * y;
            largest_area = largest_area.max(area);
        }
    }

    Ok(largest_area)
}
