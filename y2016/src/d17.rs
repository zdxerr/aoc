use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use y2015::d04_md5::md5;

pub fn bfs(input_path: &PathBuf, find_min: bool) -> Result<String, Box<dyn std::error::Error>> {
    let passcode = fs::read_to_string(input_path)?.trim().to_owned();

    let mut queue: VecDeque<((usize, usize), String)> = VecDeque::with_capacity(100);
    queue.push_back(((0, 0), "".to_string()));
    let mut max_path = "".to_string();

    while let Some((pos, path)) = queue.pop_front() {
        let (worda, _, _, _) = md5(&format!("{passcode}{path}"));

        if pos == (3, 3) {
            if find_min {
                return Ok(path);
            } else {
                if path.len() > max_path.len() {
                    max_path = path;
                }
                continue;
            }
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
    if find_min {
        Err("no solution found".into())
    } else {
        Ok(max_path)
    }
}

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    bfs(input_path, true)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    bfs(input_path, false).and_then(|path| Ok(path.len()))
}
