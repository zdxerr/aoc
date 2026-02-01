use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn skip_to_next_line(bytes: &mut impl Iterator<Item = u8>) {
    for b in bytes {
        if b == b'\n' {
            break;
        }
    }
}

fn next<T>(bytes: &mut impl Iterator<Item = u8>) -> Option<T>
where
    T: Default + Copy + From<u8> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    let mut number: Option<T> = None;
    for b in bytes {
        let digit = b - b'0';
        if (0..=9).contains(&digit) {
            let n0 = number.get_or_insert_default();
            *n0 = *n0 * T::from(10_u8) + T::from(digit);
        } else if number.is_some() {
            break;
        }
    }
    number
}

fn parse<T>(input_path: &PathBuf) -> Result<Vec<(T, T, T, T)>, Box<dyn std::error::Error>>
where
    T: Default + Copy + Ord + From<u8> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    let reader = BufReader::new(fs::File::open(input_path)?);
    let bytes = &mut reader.bytes().flatten().peekable();
    let mut nodes: Vec<(T, T, T, T)> = std::iter::repeat_with(|| {
        skip_to_next_line(bytes);
        (next(bytes), next(bytes), next(bytes), next(bytes))
    })
    .map_while(|(a, b, c, d)| {
        a.and_then(|a| b.and_then(|b| c.and_then(|c| d.and_then(|d| Some((b, a, c, d))))))
    })
    .collect();
    nodes.sort_unstable();
    Ok(nodes)
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let nodes = parse::<u32>(input_path)?;

    let mut count = 0;
    let len = nodes.len();
    for a in 0..len {
        for b in 0..len {
            if a == b {
                continue;
            }
            if nodes[a].3 > 0 && nodes[a].3 <= (nodes[b].2 - nodes[b].3) {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn _print_grid(nodes: &Vec<(u32, u32, u32, u32)>, columns: u32) {
    for x in 0..=columns {
        print!("{x:2} ");
    }
    println!();
    for (y, x, _, used) in nodes {
        if *x == 0 {
            print!("{y:3} ");
        }
        print!(
            " {} ",
            if *used == 0 {
                '_'
            } else if *used > 300 {
                '#'
            } else {
                '.'
            }
        );
        if *x == columns {
            println!();
        }
    }
}

pub fn part2(input_path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    let nodes = parse::<u32>(input_path)?;

    let columns = nodes.last().ok_or_else(|| "unable to find last row")?.1;

    // println!();
    // _print_grid(&nodes, columns);

    let left_wall_edge = nodes
        .iter()
        .find_map(|(_, x, _, used)| if *used > 300 { Some(x) } else { None })
        .ok_or_else(|| "unable to find left wall edge")?;

    let empty_node = nodes
        .iter()
        .find_map(|(y, x, _, used)| if *used == 0 { Some((x, y)) } else { None })
        .ok_or_else(|| "unable to find empty node")?;

    // print the grid and count the number of moves necessary:
    // 3 + 25 + 32 + 5*32 = 220
    Ok(empty_node.0 - left_wall_edge + 1 + empty_node.1 + (columns - 1) + 5 * (columns - 1))
}
