use core::iter::Iterator;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
// All permutations with Heaps Algorithm?

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut persons: HashMap<&str, HashMap<&str, i64>> = HashMap::with_capacity(10);
    for line in content.lines() {
        let splitted: Vec<&str> = line.split(' ').collect();
        persons.entry(splitted[0]).or_default().insert(
            splitted[splitted.len() - 1].strip_suffix('.').unwrap(),
            splitted[3].parse::<i64>()? * if splitted[2] == "lose" { -1 } else { 1 },
        );
    }
    // dbg!(&persons);

    let mut counter = vec![0; persons.len()];
    let mut a: Vec<&&str> = persons.keys().collect();

    fn calc(a: &Vec<&&str>, persons: &HashMap<&str, HashMap<&str, i64>>) -> i64 {
        (0..a.len())
            .map(|idx| {
                let left = a[(idx - 1) % a.len()];
                let this = a[idx];
                let right = a[(idx + 1) % a.len()];
                &persons[this][left] + &persons[this][right]
            })
            .sum()
    }

    // println!(" {a:?} {}", calc(&a, &persons));
    let mut happiness = calc(&a, &persons);
    let mut i = 1;
    while i < a.len() {
        if counter[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(counter[i], i);
            }
            // println!(" {a:?} {}", calc(&a, &persons));
            happiness = happiness.max(calc(&a, &persons));
            // break;

            counter[i] += 1;
            i = 1;
        } else {
            counter[i] = 0;
            i += 1;
        }
    }
    Ok(happiness)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
