use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/* Gosper's Hack */
fn iter_u32_with_exactly_n_bits(n: u32) -> impl Iterator<Item = u32> {
    // if n > 32 || n == 0 {
    //     return std::iter::empty();
    // }

    // Start with the smallest number having n bits set: 000...0111...1 (n ones)
    let mut subset = (1u64 << n) - 1;
    let limit = 1u64 << 32; // 2^32

    std::iter::from_fn(move || {
        if subset >= limit {
            return None;
        }

        let current = subset as u32;

        // Gosper's hack: find the next number with the same number of 1-bits
        let smallest = subset & (!subset + 1); // lowest set bit (two's complement trick)
        let ripple = subset + smallest;
        let ones = subset ^ ripple;
        subset = ripple | ((ones >> 2) / smallest);

        Some(current)
    })
}

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut packages: Vec<u64> = content.lines().map(|line| line.parse()).flatten().collect();
    let boundary = packages.iter().sum::<u64>().div_euclid(3);
    packages.sort();
    packages.reverse();

    let mut max_package_count = packages.len();
    let mut min_quantum_entaglement: u64 = packages.iter().product();

    for x in 1.. {
        for mut n in iter_u32_with_exactly_n_bits(x) {
            let package_count = n.count_ones() as usize;
            if package_count > max_package_count {
                break;
            }

            let start = n.trailing_zeros() as usize;
            let mut sum = 0;
            let mut set = HashSet::new();

            n >>= start;

            for index in start..packages.len() {
                let bit = n & 1 == 1;
                if bit {
                    let p = packages[index];
                    sum += p;
                    set.insert(p);
                }
                n >>= 1;
                if n == 0 {
                    break;
                }
            }

            if sum == boundary {
                max_package_count = package_count;
                min_quantum_entaglement = min_quantum_entaglement.min(set.iter().product());
            }
        }
        if x as usize >= max_package_count {
            break;
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

    let mut max_package_count = packages.len();
    let mut min_quantum_entaglement: u64 = packages.iter().product();

    for x in 1.. {
        for mut n in iter_u32_with_exactly_n_bits(x) {
            let package_count = n.count_ones() as usize;
            if package_count > max_package_count {
                break;
            }

            let start = n.trailing_zeros() as usize;
            let mut sum = 0;
            let mut set = HashSet::new();

            n >>= start;

            for index in start..packages.len() {
                let bit = n & 1 == 1;
                if bit {
                    let p = packages[index];
                    sum += p;
                    set.insert(p);
                }
                n >>= 1;
                if n == 0 {
                    break;
                }
            }

            if sum == boundary {
                max_package_count = package_count;
                min_quantum_entaglement = min_quantum_entaglement.min(set.iter().product());
            }
        }
        if x as usize >= max_package_count {
            break;
        }
    }

    Ok(min_quantum_entaglement)
}
