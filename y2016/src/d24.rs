use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;

fn _print_map(map: &[u8], xlen: usize) {
    for index in (0..map.len()).step_by(xlen) {
        println!("{}", String::from_utf8_lossy(&map[index..index + xlen - 1]));
    }
}

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn permute<T, F>(a: &mut [T], mut visit: F)
where
    F: FnMut(&[T]),
{
    fn inner<T, F>(a: &mut [T], n: usize, visit: &mut F)
    where
        F: FnMut(&[T]),
    {
        if n == 1 {
            visit(a);
            return;
        }
        inner(a, n - 1, visit);
        for i in 0..n - 1 {
            if n % 2 == 0 {
                a.swap(i, n - 1);
            } else {
                a.swap(0, n - 1);
            }
            inner(a, n - 1, visit);
        }
    }
    inner(a, a.len(), &mut visit);
}

pub fn solve(input_path: &PathBuf, part2: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let map = fs::read(input_path)?;
    // let map = fs::read("input/y2016/d24/test.txt")?;
    let xlen = map
        .iter()
        .position(|&c| c == b'\n')
        .ok_or_else(|| "unable to find new-line")?
        + 1;

    let positions = map
        .iter()
        .enumerate()
        .fold([None; 8], |mut positions, (index, value)| {
            if (b'0'..=b'9').contains(value) {
                positions[(value - b'0') as usize] = Some(index);
            }
            positions
        });
    let positions_len = positions.iter().flatten().count();

    // _print_map(&map, xlen);

    let mut queue = VecDeque::with_capacity(map.len());
    let mut visited = vec![0; map.len()];
    let mut shortest_paths = HashMap::with_capacity(16);
    for a in 0..positions_len {
        let from = positions[a].ok_or_else(|| format!("missing position: {a}"))?;
        queue.push_back((from, 0));
        visited[from] = from;

        while let Some((index, len)) = queue.pop_front() {
            let value = map[index];
            if (b'0'..=b'9').contains(&value) {
                let b = value - b'0';
                shortest_paths.insert(1 << a | 1 << b, len);
            }

            for next in [index - xlen, index + 1, index + xlen, index - 1] {
                if map[next] != b'#' && visited[next] != from {
                    visited[next] = from;
                    queue.push_back((next, len + 1));
                }
            }
        }
    }

    let mut v: Vec<usize> = (1..positions_len).collect();
    let mut min = usize::MAX;
    permute(&mut v[..], |route| {
        let len: usize = shortest_paths[&(1 | 1 << route[0])]
            + route
                .windows(2)
                .map(|positions| shortest_paths[&(1 << positions[0] | 1 << positions[1])])
                // .flatten()
                .sum::<usize>()
            + if part2 {
                shortest_paths[&(1 | 1 << route.last().unwrap())]
            } else {
                0
            };
        min = min.min(len);
    });

    Ok(min)
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, false)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    solve(input_path, true)
}
