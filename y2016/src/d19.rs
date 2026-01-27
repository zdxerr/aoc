use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // https://en.wikipedia.org/wiki/Josephus_problem
    let number_of_elves: usize = fs::read_to_string(input_path)?.trim().parse()?;
    Ok(
        !(1 << (64 - number_of_elves.strict_mul(2).leading_zeros() - 1))
            & ((number_of_elves << 1) | 1),
    )
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let number_of_elves: usize = fs::read_to_string(input_path)?.trim().parse()?;
    // let number_of_elves: usize = 5;

    let pivot = number_of_elves.div_euclid(2);
    let mut left: VecDeque<usize> = VecDeque::from_iter(1..=pivot);
    let mut right: VecDeque<usize> = VecDeque::from_iter(pivot + 1..=number_of_elves);

    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }

        // rotate to move to the next elf
        if let Some(idx) = left.pop_front() {
            right.push_front(idx);
        }
        if let Some(idx) = right.pop_back() {
            left.push_back(idx);
        }
    }

    if let Some(idx) = left.pop_front() {
        Ok(idx)
    } else if let Some(idx) = right.pop_front() {
        Ok(idx)
    } else {
        Err("no solution found".into())
    }
}
