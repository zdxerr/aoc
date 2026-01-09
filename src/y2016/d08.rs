use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

type Display = [[bool; 50]; 6];

fn _print_display(display: &Display) {
    let s: String = display
        .iter()
        .map(|row| {
            row.iter()
                .map(|column| if *column { '#' } else { '.' })
                .chain(['\n'])
        })
        .flatten()
        .collect();
    println!("{}", s);
}

fn next_usize<'a>(bytes: &mut std::slice::Iter<'a, u8>) -> Option<usize> {
    let mut num = None;
    for b in bytes {
        if (b'0'..=b'9').contains(b) {
            let num = num.get_or_insert_default();
            *num = *num * 10 + (b - b'0') as usize;
        } else if num.is_some() {
            return num;
        }
    }
    num
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut display: Display = [[false; _]; _];
    let mut cache = [false; 6];

    for line in reader.split(b'\n').flatten() {
        match &line[..4] {
            b"rect" => {
                let line_iter = &mut line[4..].iter();
                if let (Some(x1), Some(y1)) = (next_usize(line_iter), next_usize(line_iter)) {
                    for x in 0..x1 {
                        for y in 0..y1 {
                            display[y][x] = true;
                        }
                    }
                } else {
                    return Err(format!("invalid command {}", stringify!(line)).into());
                }
            }
            b"rota" => {
                let line_iter = &mut line[10..].iter();
                match &line[7..10] {
                    b"row" => {
                        if let (Some(y1), Some(shift)) =
                            (next_usize(line_iter), next_usize(line_iter))
                        {
                            display[y1].rotate_right(shift);
                        } else {
                            return Err(format!("invalid command {}", stringify!(line)).into());
                        }
                    }
                    b"col" => {
                        if let (Some(x1), Some(shift)) =
                            (next_usize(line_iter), next_usize(line_iter))
                        {
                            (0..display.len()).for_each(|index| cache[index] = display[index][x1]);
                            //     .map(|index| display[index][x1])
                            //     .try_into()?;
                            // let column: Vec<bool> =
                            //     (0..display.len()).map(|index| display[index][x1]).collect();

                            for index in 0..display.len() {
                                display[(index + shift).rem_euclid(display.len())][x1] =
                                    cache[index];
                            }
                        } else {
                            return Err(format!("invalid command {}", stringify!(line)).into());
                        }
                    }
                    _ => return Err(format!("invalid command {}", stringify!(line)).into()),
                }
            }
            _ => return Err(format!("invalid command {}", stringify!(line)).into()),
        }
        // println!();
        // _print_display(&display);
    }
    Ok(display.iter().flatten().filter(|&&c| c).count()) // 117 too low
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // ###..#..#.###..#..#..##..####..##..####..###.#....
    // #..#.#..#.#..#.#..#.#..#.#....#..#.#......#..#....
    // #..#.#..#.#..#.#..#.#....###..#..#.###....#..#....
    // ###..#..#.###..#..#.#....#....#..#.#......#..#....
    // #.#..#..#.#.#..#..#.#..#.#....#..#.#......#..#....
    // #..#..##..#..#..##...##..####..##..####..###.####.

    // RURUCEOEIL
    Err("not implemented".into())
}
