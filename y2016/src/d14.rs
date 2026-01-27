use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use y2015::d04_md5::md5;

const HEX: [u8; 16] = *b"0123456789abcdef";

#[inline(always)]
fn u32_to_hex(n: u32, buf: &mut [u8]) {
    let bytes = n.to_be_bytes();
    // unrolled  helps some compilers avoid loop overhead
    buf[0] = HEX[(bytes[0] >> 4) as usize];
    buf[1] = HEX[(bytes[0] & 0x0f) as usize];
    buf[2] = HEX[(bytes[1] >> 4) as usize];
    buf[3] = HEX[(bytes[1] & 0x0f) as usize];
    buf[4] = HEX[(bytes[2] >> 4) as usize];
    buf[5] = HEX[(bytes[2] & 0x0f) as usize];
    buf[6] = HEX[(bytes[3] >> 4) as usize];
    buf[7] = HEX[(bytes[3] & 0x0f) as usize];
    // unsafe { std::str::from_utf8_unchecked(buf) }
}

#[inline(always)]
fn md5_to_hex(words: &(u32, u32, u32, u32), buffer: &mut [u8; 32]) {
    u32_to_hex(words.0, &mut buffer[0..8]);
    u32_to_hex(words.1, &mut buffer[8..16]);
    u32_to_hex(words.2, &mut buffer[16..24]);
    u32_to_hex(words.3, &mut buffer[24..]);
}

pub fn find_key(input_path: &PathBuf, stretch: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().to_string();
    // let prefix = "abc";

    let mut triples: Vec<(usize, u8)> = Vec::with_capacity(10000);
    let mut keys = HashSet::with_capacity(64);
    let mut max_key = 0;

    let mut number = 0;

    let mut hex = [0u8; 32];

    while keys.len() < 64 || number < max_key + 1000 {
        let key = format!("{prefix}{number}");
        let (worda, wordb, wordc, wordd) = md5(&key);

        let mut hex = format!("{worda:08x}{wordb:08x}{wordc:08x}{wordd:08x}");

        // md5_to_hex(md5(&key), hex);

        if stretch {
            for _ in 0..2016 {
                let (worda, wordb, wordc, wordd) = md5(&hex);
                hex = format!("{worda:08x}{wordb:08x}{wordc:08x}{wordd:08x}")
            }
        }

        let bytes = hex.as_bytes();
        let mut found_triple = false;
        for idx in 0..bytes.len() - 2 {
            let c = bytes[idx];
            if bytes[idx + 1] == c && bytes[idx + 2] == c {
                if idx + 4 < bytes.len() && bytes[idx + 3] == c && bytes[idx + 4] == c {
                    for triple in &triples {
                        if triple.1 == c && triple.0 < number && number <= triple.0 + 1000 {
                            keys.insert(triple.0);
                            max_key = max_key.max(triple.0);
                        }
                    }
                }
                if !found_triple {
                    triples.push((number, c));
                    found_triple = true;
                }
            }
        }
        number += 1;
    }

    let mut keys: Vec<_> = keys.into_iter().collect();
    keys.sort_unstable();

    // for (n, k) in keys.iter().enumerate() {
    //     println!("{n:06} {k}");
    // }
    if let Some(key) = keys.get(63) {
        Ok(*key)
    } else {
        Err("key not forund".into())
    }
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    find_key(input_path, false)
}
pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    find_key(input_path, true)
}
