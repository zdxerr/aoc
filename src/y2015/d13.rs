use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn total_happiness(seats: &[&str], persons: &HashMap<&str, HashMap<&str, i64>>) -> (i64, i64) {
    // returns sum of all links and smallest link
    (0..seats.len())
        .map(|idx| &persons[&seats[idx]][&seats[(idx + 1).rem_euclid(seats.len())]])
        .fold((0, i64::MAX), |(sum, min), link| {
            (sum + link, min.min(*link))
        })
}

fn solve(input_path: &PathBuf) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut persons: HashMap<&str, HashMap<&str, i64>> = HashMap::new();
    for line in content.lines() {
        let splitted: Vec<&str> = line.split([' ', '.']).collect();
        let value = splitted[3].parse::<i64>()? * if splitted[2] == "lose" { -1 } else { 1 };
        persons
            .entry(splitted[0])
            .or_default()
            .entry(splitted[10])
            .and_modify(|v| *v += value)
            .or_insert(value);
        persons
            .entry(splitted[10])
            .or_default()
            .entry(splitted[0])
            .and_modify(|v| *v += value)
            .or_insert(value);
    }

    let mut seats: Vec<_> = persons.keys().map(|k| *k).collect();
    let mut happiness = total_happiness(&seats, &persons);
    let mut i = 2; // start at position two for all permutations with Heap's Algorithm
    let mut counter = vec![0; persons.len()];
    while i < seats.len() {
        if counter[i] < i {
            if i % 2 == 0 {
                seats.swap(0, i);
            } else {
                seats.swap(counter[i], i);
            }
            let new_happiness = total_happiness(&mut seats, &persons);
            if new_happiness.0 > happiness.0 {
                happiness = new_happiness;
            }

            counter[i] += 1;
            i = 2;
        } else {
            counter[i] = 0;
            i += 1;
        }
    }
    Ok((happiness.0, happiness.0 - happiness.1))
}

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(solve(input_path)?.0)
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(solve(input_path)?.1)
}
