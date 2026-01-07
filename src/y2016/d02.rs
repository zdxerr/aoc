use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(fs::File::open(input_path)?);
    let mut position: i8 = 5;
    let mut code = String::new();

    reader.bytes().flatten().for_each(|c| {
        position += match c {
            b'U' => match position {
                4 | 5 | 6 | 7 | 8 | 9 => -3,
                _ => 0,
            },
            b'D' => match position {
                1 | 2 | 3 | 4 | 5 | 6 => 3,
                _ => 0,
            },
            b'L' => match position {
                2 | 3 | 5 | 6 | 8 | 9 => -1,
                _ => 0,
            },
            b'R' => match position {
                1 | 2 | 4 | 5 | 7 | 8 => 1,
                _ => 0,
            },
            b'\n' => {
                code.push((position as u8 + b'0') as char);
                0
            }
            _ => panic!("invalid instruction: {:?}", c as char),
        }
    });
    Ok(code)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors
    // can behold the many fancy conference rooms and water coolers on this floor) and go to punch
    // in the code. Much to your bladder's dismay, the keypad is not at all like you imagined it.
    // Instead, you are confronted with the result of hundreds of man-hours of
    // bathroom-keypad-design meetings:

    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    // You still start at "5" and stop when you're at an edge, but given the same instructions as
    // above, the outcome is very different:

    // You start at "5" and don't move at all (up and left are both edges), ending at 5.
    // Continuing from "5", you move right twice and down three times (through "6", "7", "B", "D", "D"),
    // ending at D.
    // Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at B.
    // Finally, after five more moves, you end at 3.
    // So, given the actual keypad layout, the code would be 5DB3.

    // Using the same instructions in your puzzle input, what is the correct bathroom code?

    Err("not implemented".into())
}
