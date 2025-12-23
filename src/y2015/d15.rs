use std::fs;
use std::path::PathBuf;

type Ingredient = (i64, i64, i64, i64, i64);

// struct Ingredient {
//     capacity: i64,
//     durability: i64,
//     flavor: i64,
//     texture: i64,
//     calories: i64,
// }

// impl Ingredient {
//     fn new(line: &str) -> Self {
//         let mut splitted = line.split(' ');
//         Ingredient {
//             capacity: splitted
//                 .nth(2)
//                 .unwrap()
//                 .strip_suffix(',')
//                 .unwrap()
//                 .parse()
//                 .unwrap(),
//             durability: splitted
//                 .nth(1)
//                 .unwrap()
//                 .strip_suffix(',')
//                 .unwrap()
//                 .parse()
//                 .unwrap(),
//             flavor: splitted
//                 .nth(1)
//                 .unwrap()
//                 .strip_suffix(',')
//                 .unwrap()
//                 .parse()
//                 .unwrap(),
//             texture: splitted
//                 .nth(1)
//                 .unwrap()
//                 .strip_suffix(',')
//                 .unwrap()
//                 .parse()
//                 .unwrap(),
//             calories: splitted.nth(1).unwrap().parse().unwrap(),
//         }
//     }
// }

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
    let ingredients: Vec<Ingredient> = content
        .lines()
        .map(|line| {
            let mut splitted = line.split(' ');

            (
                splitted
                    .nth(2)
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse()
                    .unwrap(),
                splitted
                    .nth(1)
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse()
                    .unwrap(),
                splitted
                    .nth(1)
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse()
                    .unwrap(),
                splitted
                    .nth(1)
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse()
                    .unwrap(),
                splitted.nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect();

    dbg!(&ingredients);
    //

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
