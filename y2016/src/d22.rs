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
        a.and_then(|a| b.and_then(|b| c.and_then(|c| d.and_then(|d| Some((a, b, c, d))))))
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

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // let input_path = &PathBuf::from(r"input/y2016/d22/test.txt");
    let nodes = parse::<u32>(input_path)?;
    // BFS?
    //
    for node in &nodes {
        println!("{node:?}");
    }
    let last = nodes.last().unwrap();

    let rows = last.1 as usize + 1;
    println!("{} / {}", nodes.len(), (last.0 + 1) * (last.1 + 1));

    let r15_7 = nodes[rows * 15 + 7];

    println!("{rows} {r15_7:?}");

    // grid
    //
    // let grid = nodes.iter().map(())

    // nodes.last()

    Err("not implemented".into())
}
