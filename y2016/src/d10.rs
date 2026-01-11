use std::collections::VecDeque;
use std::fs;
use std::io::{BufReader, Read};
use std::iter::Peekable;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Copy)]
struct Bot {
    input: [Option<usize>; 2],
    low: Option<usize>,
    high: Option<usize>,
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

    let bytes = &mut reader.bytes().flatten();

    let mut bots = [Bot::default(); 250];

    // bot 99 gives low to bot 97 and high to bot 43
    // bot 47 gives low to output 12 and high to bot 64
    // value 41 goes to bot 204

    loop {
        match bytes.next() {
            Some(b'b') => {
                if let (Some(bot), Some(low), Some(high)) =
                    (next_usize(bytes), next_usize(bytes), next_usize(bytes))
                {
                    bots[bot].low = Some(low);
                    bots[bot].high = Some(high);
                    // println!("bot {bot} {low} {high}");
                } else {
                    return Err("invalid bot".into());
                }
            }
            Some(b'v') => {
                if let (Some(value), Some(bot)) = (next_usize(bytes), next_usize(bytes)) {
                    match bots[bot].input {
                        [None, None] => bots[bot].input[0] = Some(value),
                        [Some(value0), None] | [None, Some(value0)] => {
                            bots[bot].input = [Some(value0.min(value)), Some(value0.max(value))]
                        }
                        [Some(_), Some(_)] => {
                            return Err(format!("too many input value for bot {bot}").into());
                        }
                    }
                    // println!("value {bot} {value}");
                } else {
                    return Err("invalid bot".into());
                }
            }
            Some(_) => return Err("invalid command".into()),
            None => break,
        }
    }

    // dbg!(bots);
    //
    let mut queue: VecDeque<&Bot> = bots
        .iter()
        .filter(|b| b.input[0].is_some() && b.input[1].is_some())
        .collect();

    while let Some(bot) = queue.pop_front() {
        match bots[bot.low].input {
            [None, None] => bots[bot].input[0] = Some(value),
            [Some(value0), None] | [None, Some(value0)] => {
                bots[bot].input = [Some(value0.min(value)), Some(value0.max(value))]
            }
            [Some(_), Some(_)] => {
                return Err(format!("too many input value for bot {bot}").into());
            }
        }

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
