use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

// #[derive(Debug)]
// enum Item<'a> {
//     Generator(&'a str),
//     Microchip(&'a str),
// }
#[derive(Debug, Clone)]
enum Item<'a> {
    Generator(&'a str),
    Microchip(&'a str),
}

// type State<'a T> = (usize, Vec<Vec<&'a Item<'a>>>);
type Floors<T> = Vec<Vec<T>>;
type State<T> = (usize, Floors<T>);

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    // let content = fs::read_to_string(input_path)?;
    let mut queue: VecDeque<State<&Item>> = VecDeque::with_capacity(1000);
    let content = fs::read_to_string(r"input/y2016/d11/test.txt")?;

    let init_floors: Floors<Item> = content
        .lines()
        .map(|line| {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            splitted
                .iter()
                .enumerate()
                .filter_map(|(pos, s)| {
                    s.starts_with("generator")
                        .then(|| Item::Generator(splitted[pos - 1]))
                        .or_else(|| {
                            s.starts_with("microchip").then(|| {
                                Item::Microchip(
                                    splitted[pos - 1]
                                        .split_once('-')
                                        .unwrap_or((splitted[pos - 1], ""))
                                        .0,
                                )
                            })
                        })
                })
                .collect()
        })
        .collect();
    println!();
    queue.push_back((
        0,
        init_floors
            .iter()
            .map(|floor| floor.iter().collect())
            .collect(),
    ));

    while let Some((floor, floors)) = queue.pop_front() {
        println!("Current floor: {floor} {:?}", floors[floor]);

        let current_floor = &floors[floor];

        current_floor
            .iter()
            .enumerate()
            .filter(|(_, item)| matches!(item, Item::Microchip(_)))
            .for_each(|(index, microchip)| {
                if (0..init_floors.len() - 1).contains(&floor) {
                    let next = floor + 1;
                    let mut next_floors = floors.clone();
                    next_floors[floor].remove(index);
                    next_floors[next].push(microchip);
                    println!("NEXT+ {next} {next_floors:?}");
                }
                if (1..init_floors.len()).contains(&floor) {
                    let next = floor - 1;
                    let mut next_floors = floors.clone();
                    next_floors[floor].remove(index);
                    next_floors[next].push(microchip);
                    println!("NEXT- {next} {next_floors:?}");
                }

                // [floor.]
                // println!("{element}");
            });
    }

    println!();

    // How to encode the current state in an easy to copy way?

    // let fl0: &[Vec<&str>; floors.len()] = [["test"]; 4];

    // let n = fl0.clone();
    // let v = floors.clone_from_slice(src);

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
