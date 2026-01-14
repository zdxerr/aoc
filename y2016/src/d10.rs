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

fn parse(input_path: &PathBuf, part1: bool) -> Result<usize, Box<dyn std::error::Error + 'static>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let bots = &mut [Bot::default(); 240];
    let outputs = &mut [None; 21];
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(100);

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
                            queue.push_front(bot);
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

    while let Some(bot) = queue.pop_front() {
        if part1 && let [Some(17), Some(61)] = bots[bot].input {
            return Ok(bot);
        }

        match (bots[bot].low, bots[bot].input[0]) {
            (Next::Bot(low), Some(value)) => {
                bots[bot].input[0] = None;
                queue.push_back(low);
                let input = &mut bots[low].input;
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
                        return Err(format!("overrun on bot").into());
                    }
                }
            }
            (Next::Output(output), Some(value)) if outputs[output].is_none() => {
                outputs[output] = Some(value);
            }
            _ => (),
        }

        match (bots[bot].high, bots[bot].input[1]) {
            (Next::Bot(high), Some(value)) => {
                bots[bot].input[1] = None;
                queue.push_back(high);
                let input = &mut bots[high].input;
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
                        return Err(format!("overrun on bot").into());
                    }
                }
            }
            (Next::Output(output), Some(value)) if outputs[output].is_none() => {
                outputs[output] = Some(value);
            }
            _ => (),
        }
    }
    if part1 {
        Err("bot with input 17 and 61 not found".into())
    } else {
        if let (Some(output0), Some(output1), Some(output2)) = (outputs[0], outputs[1], outputs[2])
        {
            Ok(output0 * output1 * output2)
        } else {
            Err(format!("outpus missing: {:?}", outputs).into())
        }
    }
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    parse(input_path, true)
}

pub fn part2(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    parse(input_path, false)
}
