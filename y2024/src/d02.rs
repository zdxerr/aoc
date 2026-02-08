use std::fs;
use std::io::{BufReader, Read};
use std::iter::Peekable;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum Token<T> {
    None,
    Unsigned(T),
    NewLine,
}

#[inline]
fn skip_line(bytes: &mut impl Iterator<Item = u8>) {
    while let Some(c) = bytes.next() {
        if c == b'\n' {
            break;
        }
    }
}

fn next_token<I>(bytes: &mut Peekable<I>) -> Token<i32>
where
    I: Iterator<Item = u8>,
{
    let mut number: Token<i32> = Token::None;
    while let Some(byte) = bytes.next() {
        if matches!(byte, b'\n') {
            return Token::NewLine;
        }
        if matches!(byte, b'0'..=b'9') {
            let digit = (byte - b'0') as i32;
            if let Token::Unsigned(number_internal) = number {
                number = Token::Unsigned(number_internal * 10 + digit);
            } else {
                number = Token::Unsigned(digit);
            }
        }
        if let Token::Unsigned(_) = number {
            if let Some(next_byte) = bytes.peek() {
                if !(b'0'..=b'9').contains(next_byte) {
                    return number;
                }
            } else {
                return number;
            }
        }
    }
    Token::None
}

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let _content = fs::read_to_string(input_path)?;
    let bytes = &mut BufReader::new(fs::File::open(input_path)?)
        .bytes()
        .flatten()
        .peekable();

    let mut counter = 0;
    let mut prev = Token::None;
    let mut delta: Option<i32> = None;

    while let current = next_token(bytes)
        && current != Token::None
    {
        // println!("{prev:?} -> {current:?} {} {}",);
        match (&prev, &current) {
            (Token::None, Token::Unsigned(_)) => prev = current,
            (_, Token::NewLine) => {
                delta = None;
                prev = Token::None;
                counter += 1;
            }
            (Token::Unsigned(a), Token::Unsigned(b)) => {
                let this_delta = b - a;
                if !(1..=3).contains(&this_delta.abs()) {
                    delta = None;
                    prev = Token::None;
                    skip_line(bytes);
                } else if let Some(delta_) = delta
                    && this_delta.signum() != delta_.signum()
                {
                    delta = None;
                    prev = Token::None;
                    skip_line(bytes);
                } else {
                    prev = current;
                    delta = Some(this_delta);
                }
            }
            (a, b) => return Err(format!("invalid tokens {:?} / {:?}", a, b).into()),
        }
    }

    // let mut counter = 0;
    // let mut decreasing = None;
    // let mut prev = None;
    // let mut number: Option<u32> = None;
    // let mut skip = false;
    // while let Some(byte) = reader.next() {
    //     match byte {
    //         b'\n' if skip => skip = false,
    //         _ if skip => continue,
    //         b'0'..=b'9' => {
    //             let digit = (byte - b'0') as u32;
    //             if let Some(number_internal) = number {
    //                 number = Some(number_internal * 10 + digit);
    //             } else {
    //                 number = Some(digit);
    //             }
    //         }
    //         _ => {
    //             if let Some(number_value) = number {
    //                 println!("{number:?}");
    //                 if let Some(prev_value) = prev {
    //                     if !(1..=3).contains(&number_value.abs_diff(prev_value)) {
    //                         skip = true;
    //                     }
    //                     if let Some(decreasing) = decreasing {
    //                         if decreasing && prev_value < number_value {
    //                             skip = true;
    //                         }
    //                     } else {
    //                         decreasing = Some(prev_value > number_value)
    //                     }
    //                 }
    //                 prev = number;
    //                 number = None;
    //             }
    //             if byte == b'\n' {
    //                 counter += 1;
    //                 decreasing = None;
    //                 prev = None;
    //                 println!("NL");
    //             }
    //         }
    //     }
    // if matches!(byte, b'0'..=b'9') {
    //     let digit = (byte - b'0') as u32;
    //     if let Some(number_internal) = number {
    //         number = Some(number_internal * 10 + digit);
    //     } else {
    //         number = Some(digit);
    //     }
    // } else if number.is_some() {
    //     println!("{number:?}");
    //     number = None;
    // }
    // }
    Ok(counter)
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
