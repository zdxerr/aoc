use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    let mut lines = content.lines().rev();

    let operators: Vec<&str> = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .filter(|column| *column != " ")
        .collect();

    let sum = lines
        .map(|row| {
            row.trim()
                .split_whitespace()
                .filter(|column| *column != " ")
                .map(|column| column.parse().unwrap())
                .collect()
        })
        .reduce(|row_a: Vec<u64>, row_b| {
            row_a
                .iter()
                .zip(row_b.iter())
                .enumerate()
                .map(|(index, (a, b))| match operators[index] {
                    "+" => a + b,
                    "*" => a * b,
                    _ => panic!("unexpected operator"),
                })
                .collect()
        })
        .unwrap()
        .iter()
        .sum();

    Ok(sum)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    let mut lines = content.lines().rev();

    let operators: Vec<(usize, char)> = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, column)| *column != ' ')
        .collect();

    println!("{operators:?}");

    let mut columns: Vec<Vec<Vec<char>>> = vec![vec![vec![]]; operators.len()];

    lines.enumerate().for_each(|(line_idx, line)| {
        println!("{line}");
        for (column_idx, op) in operators.windows(2).enumerate() {
            for char_idx in op[0].0..op[1].0 {
                let c = line.chars().nth(char_idx).unwrap();

                if let Some(cs) = columns[column_idx].get_mut(char_idx - op[0].0) {
                    cs.push(c);
                } else {
                    columns[column_idx].insert(char_idx - op[0].0, vec![c]);
                }
                println!("{char_idx} {c}");
            }
        }
    });

    println!("{columns:?}");

    // lcet sum: Vec<(usize, Vec<u64>)> = lines
    //     .map(|row| {
    //         row.trim()
    //             .split_whitespace()
    //             .filter(|column| *column != " ")
    //             .enumerate()
    //             .map(|(index, column)| {
    //                 column
    //                     .chars()
    //                     .map(|c| c.to_digit(10).unwrap())
    //                     .reduce(|a, b| match operators[index] {
    //                         "+" => a + b,
    //                         "*" => a * b,
    //                         _ => panic!("unexpected operator"),
    //                     })
    //                     .unwrap() as u64
    //             })
    //             .collect::<Vec<u64>>()
    //     })
    //     .enumerate()
    //     .inspect(|(row_idx, columns)| {
    //         println!("{row_idx}  {columns:?}");
    //     })
    //     .collect();
    // .reduce(|(index_a, row_a), (index_b, row_b)| {
    //     row_a
    //         .iter()
    //         .zip(row_b.iter())
    //         .enumerate()
    //         .map(|(index, (a, b))| match operators[index] {
    //             "+" => a + b,
    //             "*" => a * b,
    //             _ => panic!("unexpected operator"),
    //         })
    //         .collect()
    // })
    // .unwrap()
    // .iter()
    // .sum();
    Ok(9)
}
