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
                columns[column_idx][rchar_idx] =
                    columns[column_idx][rchar_idx] * 10 + c.to_digit(10).unwrap() as u64;
            }
        }
    });

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
        .sum();
    Ok(sum)
}
