use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[inline(always)]
fn parse_coord(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut grid = vec![vec![false; 1000]; 1000];
    reader.lines().map(|line| line.unwrap()).for_each(|line| {
        let mut parts = line.trim().rsplitn(4, ' ');
        let (x1, y1) = parse_coord(parts.next().unwrap());
        parts.next();
        let (x0, y0) = parse_coord(parts.next().unwrap());

        match parts.next().unwrap() {
            "turn on" => (x0..x1 + 1).for_each(|x| (y0..y1 + 1).for_each(|y| grid[x][y] = true)),
            "turn off" => (x0..x1 + 1).for_each(|x| (y0..y1 + 1).for_each(|y| grid[x][y] = false)),
            "toggle" => (x0..x1 + 1).for_each(|x| (y0..y1 + 1).for_each(|y| grid[x][y] ^= true)),
            _ => panic!("x"),
        }
    });
    Ok(grid.iter().flatten().filter(|light| **light).count())
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    let mut grid = vec![vec![0_u64; 1000]; 1000];
    reader.lines().map(|line| line.unwrap()).for_each(|line| {
        let mut parts = line.trim().rsplitn(4, ' ');
        let (x1, y1) = parse_coord(parts.next().unwrap());
        parts.next();
        let (x0, y0) = parse_coord(parts.next().unwrap());

        match parts.next().unwrap() {
            "turn on" => (x0..x1 + 1)
                .for_each(|x| (y0..y1 + 1).for_each(|y| grid[x][y] = grid[x][y].saturating_add(1))),
            "turn off" => (x0..x1 + 1)
                .for_each(|x| (y0..y1 + 1).for_each(|y| grid[x][y] = grid[x][y].saturating_sub(1))),
            "toggle" => (x0..x1 + 1)
                .for_each(|x| (y0..y1 + 1).for_each(|y| grid[x][y] = grid[x][y].saturating_add(2))),
            _ => panic!("x"),
        }
    });

    Ok(grid.iter().flatten().sum())
}
