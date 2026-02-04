use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
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

    dbg!(positions);

    println!();
    _print_map(&map, xlen);

    fn shortest_path(
        map: &Vec<u8>,
        xlen: usize,
        from: usize,
        to: usize,
        queue: &mut BinaryHeap<Reverse<(usize, usize, usize)>>,
        visited: &mut Vec<bool>,
    ) -> Option<usize> {
        queue.clear();
        visited.fill(false);

        let distance = from.rem_euclid(xlen).abs_diff(to.rem_euclid(xlen))
            + from.div_euclid(xlen).abs_diff(to.div_euclid(xlen));

        // println!(
        //     "From: {from} To: {to} Distance: {distance} {} {}",
        //     from.div_euclid(xlen).abs_diff(to.div_euclid(xlen)),
        //     from.rem_euclid(xlen).abs_diff(to.rem_euclid(xlen))
        // );

        queue.push(Reverse((0, distance, from)));

        while let Some(Reverse((len, _, index))) = queue.pop() {
            visited[index] = true;
            if index == to {
                return Some(len);
            }

            [index - xlen, index + 1, index + xlen, index - 1]
                .iter()
                .for_each(|&next| {
                    if map[next] != b'#' && !visited[next] {
                        let distance = next.rem_euclid(xlen).abs_diff(to.rem_euclid(xlen))
                            + next.div_euclid(xlen).abs_diff(to.div_euclid(xlen));
                        queue.push(Reverse((len + 1, distance, next)));
                    }
                });
        }
        None
    }

    let mut visited = vec![false; map.len()];
    let mut queue = BinaryHeap::with_capacity(1_000);
    let mut shortest_paths = HashMap::with_capacity(16);
    // find the shortest path for each unique pair
    // select the shortest sum of paths?
    for a in 0..positions_len {
        for b in (a + 1)..positions_len {
            println!("{a}->{b}");
            shortest_paths.insert(
                1 << a | 1 << b,
                shortest_path(
                    &map,
                    xlen,
                    positions[a].unwrap(),
                    positions[b].unwrap(),
                    &mut queue,
                    &mut visited,
                ),
            );
        }
    }

    let mut v: Vec<usize> = (1..positions_len).collect();
    let mut min = usize::MAX;
    permute(&mut v[..], |route| {
        let len: usize = shortest_paths[&(1 | 1 << route[0])].unwrap()
            + route
                .windows(2)
                .flat_map(|positions| shortest_paths[&(1 << positions[0] | 1 << positions[1])])
                // .flatten()
                .sum::<usize>()
            + shortest_paths[&(1 | 1 << route.last().unwrap())].unwrap(); // part 2
        min = min.min(len);
        println!("ROUTE 0 -> {route:?} {len}");
    });

    Ok(min)
    // Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
