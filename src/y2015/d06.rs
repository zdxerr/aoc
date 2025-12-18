use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[inline(always)]
fn parse_coord(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn turn_on(light: &mut bool) {
    *light = true;
}
fn turn_off(light: &mut bool) {
    *light = false;
}
fn toggle(light: &mut bool) {
    *light ^= true;
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    println!("TEST");
    let reader = BufReader::new(File::open(input_path)?);
    let mut grid = [[false; 1000]; 1000];
    reader.lines().map(|line| line.unwrap()).for_each(|line| {
        let mut parts = line.trim().rsplitn(4, ' ');
        let (x1, y1) = parse_coord(parts.next().unwrap());
        parts.next();
        let (x0, y0) = parse_coord(parts.next().unwrap());

        let func = match parts.next().unwrap() {
            "turn on" => turn_on,
            "turn off" => turn_off,
            "toggle" => toggle,
            _ => panic!("x"),
        };

        for x in x0..x1 + 1 {
            for y in y0..y1 + 1 {
                (func)(&mut grid[x][y]);
            }
        }
    });

    Ok(grid.as_flattened().iter().filter(|light| **light).count())
    // Err("not implemented".into())
}

type Grid = [i64; 1000 * 1000];

fn inc(grid: &mut Grid, x: usize, y: usize) {
    grid[x * 1000 + y] += 1;
}
fn dec(grid: &mut Grid, x: usize, y: usize) {
    grid[x * 1000 + y] -= 1;
}
fn inc2(grid: &mut Grid, x: usize, y: usize) {
    grid[x * 1000 + y] += 2;
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(input_path)?);
    println!("TEST");
    let mut grid: Grid = [0; 1000 * 1000];
    println!("TEST");
    reader
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .for_each(|(n, line)| {
            let mut parts = line.trim().rsplitn(4, ' ');
            let (x1, y1) = parse_coord(parts.next().unwrap());
            parts.next();
            let (x0, y0) = parse_coord(parts.next().unwrap());

            let func = match parts.next().unwrap() {
                "turn on" => inc,
                "turn off" => dec,
                "toggle" => inc2,
                _ => panic!("x"),
            };

            // println!("{n} {func:?}");
            println!("{n} {x0} {y0} . {x1} {y1} {func:?}");
            for x in x0..x1 + 1 {
                for y in y0..y1 + 1 {
                    (func)(&mut grid, x, y);
                    // func(&mut grid, x, y);
                    // print!("{x}/{y} ");
                }
                println!();
            }
        });

    // Ok(grid.as_flattened().iter().sum())
    Err("not implemented".into())
}
