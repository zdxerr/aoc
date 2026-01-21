use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::{BuildHasher, Hasher};
use std::path::PathBuf;
const ELEMENTS: u64 = 7;

/// Very fast hasher tuned for u64 keys
#[derive(Clone)]
pub struct FastU64Hasher {
    state: u64,
}

impl Default for FastU64Hasher {
    #[inline]
    fn default() -> Self {
        FastU64Hasher { state: 0 }
    }
}

impl Hasher for FastU64Hasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        // Final avalanche mix — good quality for 64→64
        let mut z = self.state;
        z ^= z >> 33;
        z = z.wrapping_mul(0x9e3779b97f4a7c15); // golden ratio based
        z ^= z >> 29;
        z = z.wrapping_mul(0xcaf649c8f95f1a23);
        z ^= z >> 32;
        z
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        // Fallback for non-u64 — not performance critical if you mostly use u64
        let mut hash = self.state;
        for &b in bytes {
            hash = hash.wrapping_mul(0x517cc1b727220a95).wrapping_add(b as u64);
        }
        self.state = hash;
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        // Core fast path: single high-quality multiplication
        // (this constant is widely used in fast non-crypto hashes)
        self.state = i.wrapping_mul(0x517cc1b727220a95);
    }
}

// Builder — can be used as the hasher for HashSet / HashMap
#[derive(Clone, Default, Copy)]
pub struct FastU64BuildHasher;

impl BuildHasher for FastU64BuildHasher {
    type Hasher = FastU64Hasher;

    #[inline]
    fn build_hasher(&self) -> FastU64Hasher {
        FastU64Hasher::default()
    }
}

#[inline]
fn current_index(floors: u64) -> u64 {
    floors >> (ELEMENTS * 8)
}

#[inline]
fn generators(floors: u64, index: u64) -> u64 {
    (floors >> (index * ELEMENTS * 2)) & ((1 << ELEMENTS) - 1)
}

#[inline]
fn microchips(floors: u64, index: u64) -> u64 {
    (floors >> (index * ELEMENTS * 2 + ELEMENTS)) & ((1 << ELEMENTS) - 1)
}

#[inline]
fn valid(floors: u64) -> bool {
    for floor in 0..4 {
        let generators = generators(floors, floor);
        let microchips = microchips(floors, floor);
        let difference = generators ^ microchips;
        if difference > 0 && microchips & difference > 0 && generators & difference > 0 {
            return false;
        }
    }
    true
}

fn _print_state(floors: u64) {
    let current_index = current_index(floors);
    let valid = valid(floors);
    for index in 0..4 {
        println!(
            "{} {}  {:07b}  {:07b} {}",
            index,
            if current_index == index { '#' } else { ' ' },
            generators(floors, index),
            microchips(floors, index),
            if valid { ' ' } else { 'X' },
        )
    }
}

pub fn solve(
    input_path: &PathBuf,
    added_elements: &[&str],
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    // let content = fs::read_to_string(r"input/y2016/d11/test.txt")?;

    let mut elements = HashMap::new();
    // since we got 4 floors and maximally 7 elements we can store the full state in an u64
    let mut init_floors: u64 = content.lines().fold(0_u64, |floors, line| {
        let splitted: Vec<&str> = line.split_whitespace().collect();
        floors << (ELEMENTS * 2)
            | splitted.windows(2).fold(0, |floor, words| {
                match *&words[1].trim_end_matches(['.', ',']) {
                    "generator" => {
                        let len = elements.len();
                        let element = elements.entry(words[0]).or_insert_with(|| 1 << len);
                        floor | *element
                    }
                    "microchip" => {
                        let len = elements.len();
                        let element = elements
                            .entry(words[0].trim_end_matches("-compatible"))
                            .or_insert_with(|| 1 << len);
                        floor | *element << ELEMENTS
                    }
                    _ => floor,
                }
            })
    }) | (3 << (ELEMENTS * 8));

    for element in added_elements {
        let len = elements.len();
        let element = elements.entry(element).or_insert_with(|| 1 << len);
        init_floors |= (*element | (*element << ELEMENTS)) << (ELEMENTS * 6);
    }
    // _print_state2(init_floors);

    let mut queue: VecDeque<(usize, u64)> = VecDeque::with_capacity(8192);
    let mut visited = HashSet::with_capacity_and_hasher(8192, FastU64BuildHasher);
    queue.push_back((0, init_floors));
    while let Some((step, floors)) = queue.pop_front() {
        if visited.contains(&floors) {
            continue;
        }
        if floors & (((1 << ELEMENTS * 6) - 1) << (ELEMENTS * 2)) == 0 {
            return Ok(step);
        }
        visited.insert(floors);

        let current_floor = current_index(floors);

        for microchip_index0 in 0..elements.len() {
            let microchip0 = 1 << microchip_index0;
            let microchips_current_floor = microchips(floors, current_floor);
            if !(microchips_current_floor & microchip0 > 0) {
                continue;
            }

            let next_floor = current_floor - 1;
            if (0..4).contains(&next_floor) {
                let mut next_floors =
                    (floors & !(0b11 << (ELEMENTS * 8))) | (next_floor << (ELEMENTS * 8));

                next_floors ^= microchip0 << (current_floor * 2 * ELEMENTS + ELEMENTS);
                next_floors ^= microchip0 << (next_floor * 2 * ELEMENTS + ELEMENTS);

                for microchip_index1 in microchip_index0 + 1..elements.len() {
                    let microchip1 = 1 << microchip_index1;
                    if !(microchips_current_floor & microchip1 > 0) {
                        continue;
                    }
                    let mut next_floors = next_floors;
                    next_floors ^= microchip1 << (current_floor * 2 * ELEMENTS + ELEMENTS);
                    next_floors ^= microchip1 << (next_floor * 2 * ELEMENTS + ELEMENTS);
                    if valid(next_floors) {
                        queue.push_back((step + 1, next_floors));
                    }
                }

                if generators(next_floors, current_floor) & microchip0 > 0 {
                    next_floors ^= microchip0 << (current_floor * 2 * ELEMENTS);
                    next_floors ^= microchip0 << (next_floor * 2 * ELEMENTS);
                }
                if valid(next_floors) {
                    queue.push_back((step + 1, next_floors));
                }
            }

            let next_floor = current_floor + 1;
            if (0..4).contains(&next_floor) {
                let mut next_floors =
                    (floors & !(0b11 << (ELEMENTS * 8))) | (next_floor << (ELEMENTS * 8));
                next_floors ^= microchip0 << (current_floor * 2 * ELEMENTS + ELEMENTS);
                next_floors ^= microchip0 << (next_floor * 2 * ELEMENTS + ELEMENTS);
                if valid(next_floors) {
                    queue.push_back((step + 1, next_floors));
                }
            }
        }
    }

    Err("not soultion found".into())
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, &[])
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, &["elerium", "dilithium"])
}
