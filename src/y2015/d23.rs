use std::fs;
use std::path::PathBuf;

fn solve(
    input_path: &PathBuf,
    mut index: usize,
    mut a: u64,
    mut b: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let program: Vec<&str> = content.lines().collect();

    while let Some(instruction) = program.get(index) {
        match &instruction[..3] {
            "hlf" => {
                match &instruction[4..5] {
                    "a" => a /= 2,
                    "b" => b /= 2,
                    _ => panic!("unexpected register"),
                }
                index += 1;
            }
            "tpl" => {
                match &instruction[4..5] {
                    "a" => a *= 3,
                    "b" => b *= 3,
                    _ => panic!("unexpected register"),
                }
                index += 1;
            }
            "inc" => {
                match &instruction[4..5] {
                    "a" => a += 1,
                    "b" => b += 1,
                    _ => panic!("unexpected register"),
                }
                index += 1;
            }
            "jmp" => {
                let stride = instruction[5..].parse::<usize>()?;
                index = match &instruction[4..5] {
                    "+" => index.saturating_add(stride),
                    "-" => index.saturating_sub(stride),
                    _ => panic!("unexpected sign"),
                };
            }
            "jie" => {
                if match &instruction[4..5] {
                    "a" => a,
                    "b" => b,
                    _ => panic!("unexpected register"),
                }
                .rem_euclid(2)
                    == 0
                {
                    let stride: usize = instruction[8..].parse()?;
                    index = match &instruction[7..8] {
                        "+" => index.saturating_add(stride),
                        "-" => index.saturating_sub(stride),
                        _ => panic!("unexpected sign"),
                    };
                } else {
                    index += 1;
                }
            }
            "jio" => {
                if 1 == match &instruction[4..5] {
                    "a" => a,
                    "b" => b,
                    _ => panic!("unexpected register"),
                } {
                    let stride: usize = instruction[8..].parse()?;
                    index = match &instruction[7..8] {
                        "+" => index.saturating_add(stride),
                        "-" => index.saturating_sub(stride),
                        _ => panic!("unexpected sign"),
                    };
                } else {
                    index += 1;
                }
            }
            _ => panic!("unexpected instruction: {instruction} [{index}]"),
        }
    }
    Ok(b)
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    solve(input_path, 0, 0, 0)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    solve(input_path, 0, 1, 0)
}
