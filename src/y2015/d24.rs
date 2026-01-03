use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut packages: Vec<u64> = content.lines().map(|line| line.parse()).flatten().collect();
    let boundary = packages.iter().sum::<u64>().div_euclid(3);
    packages.sort();
    packages.reverse();

    let mut range: Vec<_> = (1..2_usize.pow(packages.len() as u32)).collect();
    range.sort_unstable_by_key(|v| v.count_ones());

    let mut max_package_count = packages.len();
    let mut min_quantum_entaglement: u64 = packages.iter().product();

    for n in range.iter_mut() {
        let package_count = n.count_ones() as usize;
        if package_count > max_package_count {
            break;
        }

        let start = n.trailing_zeros() as usize;
        let mut sum = 0;
        let mut set = HashSet::new();

        *n >>= start;

        for index in start..packages.len() {
            let bit = *n & 1 == 1;
            if bit {
                let p = packages[index];
                sum += p;
                set.insert(p);
            }
            *n >>= 1;
            if *n == 0 {
                break;
            }
        }

        if sum == boundary {
            max_package_count = package_count;
            min_quantum_entaglement = min_quantum_entaglement.min(set.iter().product());
        }
    }

    Ok(min_quantum_entaglement)
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut packages: Vec<u64> = content.lines().map(|line| line.parse()).flatten().collect();
    let boundary = packages.iter().sum::<u64>().div_euclid(4);
    packages.sort();
    packages.reverse();

    let mut range: Vec<_> = (1..2_usize.pow(packages.len() as u32)).collect();
    range.sort_unstable_by_key(|v| v.count_ones());

    let mut max_package_count = packages.len();
    let mut min_quantum_entaglement: u64 = packages.iter().product();

    for n in range.iter_mut() {
        let package_count = n.count_ones() as usize;
        if package_count > max_package_count {
            break;
        }

        let start = n.trailing_zeros() as usize;
        let mut sum = 0;
        let mut set = HashSet::new();

        *n >>= start;

        for index in start..packages.len() {
            let bit = *n & 1 == 1;
            if bit {
                let p = packages[index];
                sum += p;
                set.insert(p);
            }
            *n >>= 1;
            if *n == 0 {
                break;
            }
        }

        if sum == boundary {
            max_package_count = package_count;
            min_quantum_entaglement = min_quantum_entaglement.min(set.iter().product());
        }
    }

    Ok(min_quantum_entaglement)
}
