use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    Ok(reader
        .split(b'\n')
        .flatten()
        .filter_map(|line| {
            let checksum = &line[line.len() - 6..line.len() - 1];
            let mut count: [(u8, u8); 26] = std::array::from_fn(|i| (0, b'a' + i as u8));
            let mut sector_id = 0;
            line[..line.len() - 7].iter().for_each(|c| {
                let idx = (c - b'a') as usize;
                let num = c - b'0';

                if let Some((count, _)) = count.get_mut(idx) {
                    *count += 1;
                } else if num < 10 {
                    sector_id = sector_id * 10 + num as u64;
                }
            });
            count.sort_by_key(|&(count, c)| (std::cmp::Reverse(count), c));
            let actual_checksum: Vec<u8> = count.iter().take(5).map(|(_, c)| *c).collect();
            if checksum == actual_checksum {
                Some(sector_id)
            } else {
                None
            }
        })
        .sum())
}

// With all the decoy data out of the way, it's time to decrypt this list and get moving.

// The room names are encrypted by a state-of-the-art shift cipher, which is nearly unbreakable
// without the right software. However, the information kiosk designers at Easter Bunny HQ were
// not expecting to deal with a master cryptographer like yourself.

// To decrypt a room name, rotate each letter forward through the alphabet a number of times equal
// to the room's sector ID. A becomes B, B becomes C, Z becomes A, and so on. Dashes become spaces.

// For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.

// What is the sector ID of the room where North Pole objects are stored?
pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    reader.split(b'\n').flatten().find(|line| {
        let mut parts = line[..line.len() - 7].rsplitn(2, |&c| c == b'-');
        let (sector_id, code) = (
            parts.next().expect("split line part 1"),
            parts.next().expect("split line part 2"),
        );
        // println!(
        //     "{:?}  {:?}",
        //     std::str::from_utf8(code).unwrap(),
        //     std::str::from_utf8(sector_id).unwrap()
        // );
        let sector_id: u32 = sector_id.iter().fold(0, |num, digit| {
            // println!(" {} {}", *digit as char, num);
            num * 10 + (digit - b'0') as u32
        });

        // println!("{sector_id:?} {code:?}");
        let new = String::from_utf8(
            code.iter()
                .map(|c| match c {
                    b'-' => b' ',
                    _ => ((c - b'a') as u32 + sector_id).rem_euclid(26) as u8 + b'a',
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap();
        if new.contains("northpole object storage") {
            println!("{new:?} {sector_id}");
        }

        // line[..line.len() - 7].iter().rev().for_each(|c| {
        //     let idx = (c - b'a') as usize;
        //     let num = c - b'0';

        //     // if let Some((count, _)) = count.get_mut(idx) {
        //     //     *count += 1;
        //     // } else if num < 10 {
        //     //     sector_id = sector_id * 10 + num as u64;
        //     // }
        // });
        false
    });

    Err("not implemented".into())
}
