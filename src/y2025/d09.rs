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

fn area(a: (u64, u64), b: (u64, u64)) -> u64 {
    let x = a.0.abs_diff(b.0) + 1;
    let y = a.1.abs_diff(b.1) + 1;
    x * y
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read(input_path)?;
    let corners: Vec<(u64, u64)> = content
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut splitted = line
                .splitn(2, ',')
                .map(|number| number.parse::<u64>().unwrap());
            (splitted.next().unwrap(), splitted.next().unwrap())
        })
        .collect();

    let len = corners.len();

    let mut edges: Vec<(u64, (u64, u64), (u64, u64))> = corners
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .map(|(a, b)| (area(a, b), a, b))
        .collect();
    edges.push({
        let (a, b) = (corners[0], corners[corners.len() - 1]);
        let (a, b) = (a.min(b), a.max(b));
        (area(a, b), a, b)
    });
    //
    edges.sort_unstable();
    edges.reverse();

    let mut sizes: Vec<(u64, (u64, u64), (u64, u64))> = vec![];

    for a in 0..len {
        for b in 0 + 1..len {
            let (a, b) = (corners[a].min(corners[b]), corners[a].max(corners[b]));
            sizes.push((area(a, b), a, b));
        }
    }
    sizes.sort_unstable();
    sizes.reverse();

    for (this_area, (x0, y0), (x1, y1)) in sizes {
        let (y0, y1) = (y0.min(y1), y0.max(y1));
        if !{
            let mut result = false;
            for (_, (x2, y2), (x3, y3)) in &edges {
                let (y2, y3) = (y2.min(y3), y2.max(y3));

                if *x3 > x0 && *x2 < x1 && *y3 > y0 && *y2 < y1 {
                    result = true;
                    break;
                }
            }
            result
        } {
            return Ok(this_area);
        }
    }
    unreachable!();
}
