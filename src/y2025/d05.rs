use std::fs;
use std::path::PathBuf;
use std::str::Lines;

fn reduce_ranges(lines: &mut Lines) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .for_each(|(mut a, b): (u64, u64)| {
            let mut index = 0;
            loop {
                if index >= ranges.len() {
                    break;
                }
                let (c, d) = ranges[index];

                if a <= c - 1 {
                    if b < c - 1 {
                        ranges.insert(index, (a, b));
                        return;
                    } else if b <= d {
                        ranges[index].0 = a;
                        return;
                    } else {
                        ranges.remove(index);
                        continue; // TODO
                    }
                } else if a <= d + 1 {
                    if b <= d {
                        return;
                    } else {
                        a = c;
                        ranges.remove(index);
                        continue;
                    }
                } else {
                    index += 1;
                    continue;
                }
            }
            ranges.push((a, b));
        });
    ranges
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut lines = content.lines();
    let ranges = reduce_ranges(&mut lines);

    let sum = lines
        .map(|line| line.trim().parse().unwrap())
        .map(|id: u64| {
            for (start, end) in &ranges {
                if (start..=end).contains(&&id) {
                    return 1;
                }

                // if id > *end {
                //     break;
                // }
                // else if id >= *start {
                //     return 1;
                // }
            }
            0
        })
        .sum();

    return Ok(sum);
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut lines = content.lines();

    let ranges = reduce_ranges(&mut lines);
    Ok(ranges.iter().map(|(a, b)| b - a + 1).sum())
}
