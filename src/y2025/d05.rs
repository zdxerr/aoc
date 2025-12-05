use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    let content = fs::read_to_string(input_path)?;

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut parse_ranges = true;

    'outer: for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            parse_ranges = false;
            continue;
        }
        if parse_ranges {
            ranges.push(
                if let Some((a, b)) = line.split_once('-') {
                    Some((a.trim().parse()?, b.trim().parse()?))
                } else {
                    None
                }
                .ok_or("unable to parse")?,
            );
        } else {
            let id: u64 = line.parse()?;
            for (start, end) in &ranges {
                if (start..=end).contains(&&id) {
                    sum += 1;
                    continue 'outer;
                }
            }
        }
    }
    Ok(sum)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    let content = fs::read_to_string(input_path)?;

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut acc: Vec<(u64, u64)> = Vec::new();
    let ranges = content
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .for_each(|(a, b)| {
            let mut p = true;
            let mut a = a;
            let mut b = b;
            for (c, d) in acc.iter_mut() {
                if a < *c {
                    if b < *c {
                        acc.insert(0, (a, b));
                        p = false;
                        break;
                    } else if b < *d {
                        *c = a;
                        p = false;
                        break;
                        // acc[acc.len() - 1] = (a, *d);
                    } else {
                        *c = a;
                        *d = b;
                        p = false;
                        break;
                        // acc[acc.len() - 1] = (a, b);
                    }
                } else if a <= *d {
                    if b <= *d {
                        p = false;
                        break;
                    } else {
                        // *d = b;
                        a = *c;
                        *c = 0;
                        *d = 0;
                        // p = false;
                        // break;
                    }
                }
            }
            if p {
                acc.push((a, b));
            }

            // println!("{a} {b} {acc:?}");
            // acc.push((a, b));
        });
    println!("{acc:?}");

    Ok(acc
        .iter()
        .filter(|(a, b)| *a != 0 && *b != 0)
        .map(|(a, b)| b - a + 1)
        .sum())

    // .fold(Vec::new(), |acc, (a, b)| {
    //     let last = acc.last();
    //     println!("{last:?} {a} {b}");
    //     acc.push((a, b));
    //     acc
    // });
    // .collect::<Vec<(u64, u64)>>();

    // 'outer: for line in content.lines() {
    //     println!("{ranges:?}");
    //     let line = line.trim();
    //     if line.is_empty() {
    //         break;
    //     }
    //     if let Some((a, b)) = line.split_once('-') {
    //         let mut n: (u64, u64) = (a.trim().parse()?, b.trim().parse()?);
    //         println!("{n:?}");
    //         for (index, r) in ranges.iter().enumerate() {
    //             if n.0 < r.0 && n.1 < r.0 {
    //                 ranges.insert(index, n);
    //                 continue 'outer;
    //             }
    //             if n.0 <= r.0 && n.1 >= r.0 && n.1 <= r.1 {
    //                 ranges[index] = (n.0, r.1);
    //                 continue 'outer;
    //             }
    //             if n.0 >= r.0 && n.1 <= r.1 {
    //                 continue 'outer;
    //             }
    //             if n.0 >= r.0 && n.0 <= r.1 && n.1 >= r.1 {
    //                 n = (r.0, n.1);
    //                 ranges.remove(index);
    //                 println!("{n:?}");
    //                 // ranges[index] = (r.0, n.1);
    //                 continue;
    //             }
    //             if n.0 > r.1 {
    //                 continue;
    //             }
    //         }
    //         ranges.push(n);
    //     }
    // }
    // Ok(sum)
}
