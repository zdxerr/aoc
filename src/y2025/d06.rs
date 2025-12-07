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

    // println!("{operators:?}");

    // let mut columns: Vec<Vec<Vec<u64>>> = vec![vec![vec![]]; operators.len()];
    let mut columns: Vec<Vec<u64>> = vec![vec![]; operators.len()];

    lines.rev().for_each(|line| {
        for column_idx in 0..operators.len() {
            let op = operators[column_idx];
            let x = if let Some(next_op) = operators.get(column_idx + 1) {
                next_op.0 - 1
            } else {
                line.len()
            };
            for char_idx in op.0..x {
                let rchar_idx = char_idx - op.0;
                let x = columns[column_idx].get(rchar_idx);
                if x == None {
                    // columns[column_idx].insert(rchar_idx, vec![]);
                    columns[column_idx].insert(rchar_idx, 0);
                }
                let c = line.chars().nth(char_idx).unwrap();
                if c == ' ' {
                    continue;
                }
                // let lindex = columns[column_idx][rchar_idx].len();
                // columns[column_idx][rchar_idx].insert(lindex, c.to_digit(10).unwrap() as u64);
                columns[column_idx][rchar_idx] =
                    columns[column_idx][rchar_idx] * 10 + c.to_digit(10).unwrap() as u64;
                // columns[column_idx][rchar_idx].push(c.to_digit(10).unwrap() as u64);

                // if let Some(cs) = columns[column_idx].get_mut(char_idx - op[0].0) {
                //     cs.push(c);
                // } else {
                //     columns[column_idx].insert(char_idx - op[0].0, vec![c]);
                // }
            }
        }
    });

    // println!("{columns:?}");
    let sum = operators
        .iter()
        .zip(columns.into_iter())
        .map(|(op, values)| {
            values
                .into_iter()
                .reduce(|acc, value| match op.1 {
                    '+' => acc + value,
                    '*' => acc * value,
                    _ => panic!(""),
                })
                .unwrap()
        })
        // .inspect(|x| {
        //     println!("  {x}");
        // })
        .sum();
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
    Ok(sum)
}
