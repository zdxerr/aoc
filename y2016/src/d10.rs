use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Copy)]
enum Next {
    Bot(usize),
    Output(usize),
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Copy)]
struct Bot {
    input: [Option<usize>; 2],
    low: Next,
    high: Next,
}

fn next_usize<'a>(
    bytes: &mut impl Iterator<Item = u8>,
    // bytes: &mut impl Peekable<Item = u8>,
) -> Option<usize> {
    let mut num = None;
    for b in bytes {
        match (b, num) {
            (b'0'..=b'9', None) => num = Some((b - b'0') as usize),
            (b'0'..=b'9', Some(num0)) => {
                num = Some(num0 * 10 + (b - b'0') as usize);
            }
            (_, Some(_)) => return num,
            _ => (),
        }
        // if (b'0'..=b'9').contains(&b) {
        //     let num = num.get_or_insert_default();
        //     *num = *num * 10 + (b - b'0') as usize;
        // } else if num.is_some() {
        //     return num;
        // }
    }
    num
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let _content = fs::read_to_string(input_path)?;
    let reader = BufReader::new(fs::File::open(input_path)?);
    // let bytes = &mut reader.bytes().flatten();

    let mut bots = [Bot::default(); 250];

    // bot 99 gives low to bot 97 and high to bot 43
    // bot 47 gives low to output 12 and high to bot 64
    // value 41 goes to bot 204
    //
    let patterns: &[&[u8]] = &[
        b"value ? goes to bot ?",
        b"bot ? gives low to bot ? and high to bot ?",
        b"bot ? gives low to output 12 and high to bot ?",
        b"bot ? gives low to bot 12 and high to output ?",
        b"bot ? gives low to output 12 and high to output ?",
    ];

    for line in reader.lines().flatten() {
        let mut splitted = line.split(' ');
        match splitted.next() {
            Some("value") => {
                if let (Some(value), Some(bot)) = (
                    splitted.next().and_then(|s| s.parse::<usize>().ok()),
                    splitted.nth(3).and_then(|s| s.parse::<usize>().ok()),
                ) {
                    let input = &mut bots[bot].input;
                    match input {
                        [None, None] => input[0] = Some(value),
                        [Some(value0), None] => {
                            if *value0 < value {
                                input[1] = Some(value);
                            } else {
                                input[1] = input[0];
                                input[0] = Some(value);
                            }
                        }
                        _ => {
                            return Err(format!(
                                "overrun with command: {} [{:?}]",
                                line, bots[bot]
                            )
                            .into());
                        }
                    }
                } else {
                    return Err(format!("unable to parse command: {line}").into());
                }
            }
            Some("bot") => {
                if let Some(bot) = splitted.next().and_then(|s| s.parse::<usize>().ok()) {
                    match (
                        splitted.nth(3),
                        splitted.next().and_then(|s| s.parse::<usize>().ok()),
                    ) {
                        (Some("bot"), Some(low)) => bots[bot].low = Next::Bot(low),
                        (Some("output"), Some(low)) => bots[bot].low = Next::Output(low),
                        _ => return Err(format!("unable to parse command: {line}").into()),
                    }
                    match (
                        splitted.nth(3),
                        splitted.next().and_then(|s| s.parse::<usize>().ok()),
                    ) {
                        (Some("bot"), Some(low)) => bots[bot].high = Next::Bot(low),
                        (Some("output"), Some(low)) => bots[bot].high = Next::Output(low),
                        _ => return Err(format!("unable to parse command: {line}").into()),
                    }
                }
            }
            _ => return Err(format!("invalid command: {line}").into()),
        }
    }

    // dbg!(bots);
    //
    let mut queue: VecDeque<&Bot> = bots
        .iter()
        .filter(|b| b.input[0].is_some() && b.input[1].is_some())
        .collect();

    while let Some(bot) = queue.pop_front() {
        println!("{bot:?}");
        // match bots[bot.low].input {
        //     [None, None] => bots[bot].input[0] = Some(value),
        //     [Some(value0), None] | [None, Some(value0)] => {
        //         bots[bot].input = [Some(value0.min(value)), Some(value0.max(value))]
        //     }
        //     [Some(_), Some(_)] => {
        //         return Err(format!("too many input value for bot {bot}").into());
        //     }
        // }

        // if let [Some(value0), None] = bots[bot.low].input
    }
    // let mut b = Bot {
    //     input: [None, None],
    //     low: 0,
    //     high: 1,
    // };

    // b.input;

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
