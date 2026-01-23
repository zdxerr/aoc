use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{PathBuf, StripPrefixError};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::thread;
use y2015::d04_md5::md5;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let prefix = fs::read_to_string(input_path)?.trim().to_string();
    let prefix = "abc";

    let mut key = String::with_capacity(20);
    key.push_str(prefix);
    let prefix_len = prefix.len();
    let mut triples: HashMap<u8, Vec<usize>> = HashMap::with_capacity(16);
    let mut keys = HashSet::with_capacity(64);
    println!();

    for number in 0_usize.. {
        // for number in 0..=30 {
        key.replace_range(prefix_len.., &number.to_string());
        let (worda, wordb, wordc, wordd) = md5(&key);
        let hash = format!("{worda:08x}{wordb:08x}{wordc:08x}{wordd:08x}");

        // println!("{key} {hash}");

        let b = hash.as_bytes();
        // let mut q = false;
        for idx in 0..b.len() - 7 {
            let c = b[idx];
            if b[idx + 1] == c && b[idx + 2] == c {
                let ctriples = triples.entry(c).or_default();

                if b[idx + 3] == c && b[idx + 4] == c {
                    println!("QUINTUPLE {} {number} ({})", c as char, hash);
                    println!("-> {ctriples:?}");
                    // let p = ctriples.partition_point(|&tnumber| tnumber > number - 1000);
                    // let r = ..p.max(1);
                    // println!("{p}, {r:?}, {:?}", &ctriples[p..]);
                    for tnumber in ctriples
                        .drain(..)
                        .rev()
                        .take_while(|&tnumber| tnumber >= number.saturating_sub(1000))
                    {
                        println!("{tnumber} .. {}", keys.len());
                        keys.insert(tnumber);
                        if keys.len() >= 64 {
                            return Ok(*keys.iter().max().unwrap());
                        }
                    }

                    // for tnumber in ctriples
                    //     .iter()
                    //     .rev()
                    //     .take_while(|&&tnumber| tnumber > number - 1000)
                    // {
                    //     keys.insert(tnumber);
                    // }
                    // quintuples.push(number);
                    // q = true;
                    // break;
                }
                match ctriples.last() {
                    Some(&last_number) if last_number != number => ctriples.push(number),
                    None => ctriples.push(number),
                    _ => (),
                }
            }
        }
    }
    // dbg!(&triples);
    //     if q {
    //         let relevant_triples = if triples[0] > number - 1000 {
    //             let p = triples.partition_point(|&tnumber| tnumber < number - 1000);
    //             &triples[p..]
    //         } else {
    //             &triples[..]
    //         };
    //         // let p = triples.partition_point(|&tnumber| tnumber < number - 1000);
    //         // println!("{number} {p:?}, {}", triples.len());
    //         // println!("{triples:?}");
    //         // let triples = triples.split_off(p);
    //         println!("{number} {relevant_triples:?}");
    //         for triple_idx in relevant_triples {
    //             valid_indices.insert(triple_idx.clone());
    //             if valid_indices.len() >= 64 {
    //                 return Ok(*triple_idx);
    //             }
    //         }

    //         triples.clear();
    //         // for triple_idx in triples
    //         //     .iter()
    //         //     .rev()
    //         //     .take_while(|&&triple_idx| triple_idx > idx - 1000)
    //         // {
    //         //     valid_indices.insert(triple_idx);
    //         //     if valid_indices.len() >= 64 {
    //         //         return Ok(*triple_idx);
    //         //     }
    //         // }
    //     }
    // }

    // dbg!(triples.len(), quintuples.len());

    // let next_number = AtomicU64::new(0);
    // let num_workers = thread::available_parallelism()?.get();
    // let mut password: u32 = 0;
    // let found = AtomicBool::new(false);

    // thread::scope(|s| {
    //     let (tx, rx): (mpsc::Sender<u32>, mpsc::Receiver<u32>) = mpsc::channel();
    //     let next_number = &next_number;
    //     let prefix = &prefix;
    //     for _ in 0..num_workers {
    //         let tx = tx.clone();
    //         let found = &found;
    //         s.spawn(move || {
    //             let mut key = String::with_capacity(20);
    //             key.push_str(prefix);
    //             let prefix_len = prefix.len();
    //             loop {
    //                 if found.load(Ordering::Relaxed) {
    //                     break;
    //                 }
    //                 let number = next_number.fetch_add(1, Ordering::Relaxed).to_string();
    //                 key.replace_range(prefix_len.., &number);
    //                 let (word, _, _, _) = md5(&key);
    //                 if word & MASK == 0 {
    //                     if let Err(_) = tx.send((word & 0x00000F00) >> 8) {
    //                         break;
    //                     }
    //                 }
    //             }
    //         });
    //     }
    //     drop(tx);
    //     for code in rx.iter().take(8) {
    //         password = password << 4 | code;
    //     }
    //     found.store(true, Ordering::Relaxed);
    // });
    // Ok(format!("{password:x}"))
    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
