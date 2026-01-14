use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum Command {
    CPY(Value, Value),
    INC(Value),
    DEC(Value),
    JNZ(Value, Value),
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Integer(i64),
    Register(usize),
    None,
}

fn next_value(bytes: &[u8]) -> Value {
    let mut value = Value::None;
    let mut sign = false;
    for b in bytes {
        match (b, value) {
            (b'a'..=b'd', Value::None) => value = Value::Register((b - b'a') as usize),
            (b'-', Value::None) => {
                sign = true;
                value = Value::Integer(0);
            }
            (b'0'..=b'9', Value::None) => value = Value::Integer((b - b'0') as i64),
            (b'0'..=b'9', Value::Integer(num0)) => {
                value = Value::Integer(num0 * 10 + (b - b'0') as i64);
            }
            (_, Value::Integer(_) | Value::Register(_)) => break,
            _ => (),
        }
    }
    if sign && let Value::Integer(num) = value {
        Value::Integer(-1 * num)
    } else {
        value
    }
}

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    // let content = fs::read_to_string(input_path)?;
    let reader = BufReader::new(fs::File::open(input_path)?);

    // let mut a = None;

    let program: Vec<Command> = reader
        .split(b'\n')
        .flatten()
        .map(|line| {
            println!(
                "{:?} {:?} __ {:?}",
                str::from_utf8(&line),
                &line[3..line.len() - 1],
                next_value(&line[3..line.len() - 1])
            );

            // line[3..].split_once(b' ');
            match &line[0..3] {
                b"cpy" => Command::CPY(next_value(&line[3..]), next_value(&line[6..])),
                b"inc" => Command::INC(next_value(&line[3..])),
                b"dec" => Command::DEC(next_value(&line[3..])),
                b"jnz" => Command::JNZ(next_value(&line[3..]), next_value(&line[6..])),
                _ => panic!("PANIC"),
            }
        })
        .collect();

    dbg!(&program);
    let mut pos = 0;
    let mut register = [0; 4];

    while let Some(cmd) = program.get(pos) {
        match cmd {
            Command::CPY(Value::Integer(value), Value::Register(index)) => {
                register[*index] = *value
            }
            Command::CPY(Value::Register(index0), Value::Register(index1)) => {
                register[*index1] = register[*index0]
            }
            Command::INC(Value::Register(index)) => register[*index] += 1,
            Command::DEC(Value::Register(index)) => register[*index] -= 1,
            Command::JNZ(Value::Register(index), Value::Integer(jump)) => {
                if register[*index] != 0 {
                    println!("JNZ {pos} {jump}, {}", pos + *jump as usize);
                    pos += *jump as usize;
                    continue;
                }
            }
            Command::JNZ(Value::Integer(value), Value::Integer(jump)) => {
                if *value != 0 {
                    println!("JNZ {pos} {jump}, {}", pos + *jump as usize);
                    pos += *jump as usize;
                    continue;
                }
            }
            _ => return Err(format!("invalid command: {cmd:?}").into()),
        }
        pos += 1;
    }
    // for c in program {
    //     println!("{c:?}");
    // }
    Ok(register[0])
    // Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // As you head down the fire escape to the monorail, you notice it didn't start; register c
    // needs to be initialized to the position of the ignition key.

    // If you instead initialize register c to be 1, what value is now left in register a?
    Err("not implemented".into())
}
