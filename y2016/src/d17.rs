use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use y2015::d04_md5::md5;

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let passcode = fs::read_to_string(input_path)?.trim().to_owned();

    // BFS
    let goal = (3, 3);

    let mut queue: VecDeque<((usize, usize), String)> = VecDeque::with_capacity(100);
    queue.push_back(((0, 0), "".to_string()));

    while let Some((pos, path)) = queue.pop_front() {
        let (worda, _, _, _) = md5(&format!("{passcode}{path}"));
        // println!(
        //     "{pos:?} {path:20} {worda:08x} {} {} {} {}",
        //     worda >> 28 > 10,
        //     ((worda >> 24) & 0xF) > 10,
        //     ((worda >> 20) & 0xF) > 10,
        //     ((worda >> 16) & 0xF) > 10
        // );

        if pos == goal {
            return Ok(path);
        }

        if pos.1 > 0 && worda >> 28 > 10 {
            let mut path = path.clone();
            path.push('U');
            queue.push_back(((pos.0, pos.1 - 1), path));
        }
        if pos.1 < 3 && (worda >> 24) & 0xF > 10 {
            let mut path = path.clone();
            path.push('D');
            queue.push_back(((pos.0, pos.1 + 1), path));
        }
        if pos.0 > 0 && (worda >> 20) & 0xF > 10 {
            let mut path = path.clone();
            path.push('L');
            queue.push_back(((pos.0 - 1, pos.1), path));
        }
        if pos.0 < 3 && (worda >> 16) & 0xF > 10 {
            let mut path = path.clone();
            path.push('R');
            queue.push_back(((pos.0 + 1, pos.1), path));
        }
    }

    Err("no solution found".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
