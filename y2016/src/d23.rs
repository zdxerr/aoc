use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum Argument {
    Register(usize),
    Integer(i64),
}

fn parse_instruction(instruction: &[u8]) -> (&[u8], Vec<Argument>) {
    let cmd_len = instruction.iter().position(|&c| c == b' ').unwrap();

    let mut arguments = Vec::new();
    let mut negative = 1;
    let mut integer = None;
    for c in &instruction[cmd_len..] {
        match c {
            b'a'..=b'd' => arguments.push(Argument::Register((c - b'a') as usize)),
            b'-' => negative = -1,
            b'0'..=b'9' if integer.is_none() => integer = Some((c - b'0') as i64),
            b'0'..=b'9' => integer = Some(integer.unwrap() * 10 + (c - b'0') as i64),
            b' ' if integer.is_some() => {
                arguments.push(Argument::Integer(negative * integer.unwrap()));
                negative = 1;
                integer = None;
            }
            b' ' => negative = 1,
            _ => panic!("unexpected character: {}", *c as char),
        }
    }
    if integer.is_some() {
        arguments.push(Argument::Integer(negative * integer.unwrap()));
    }

    (&instruction[..cmd_len], arguments)
}

fn run(input_path: &PathBuf, init_value: i64) -> Result<i64, Box<dyn std::error::Error>> {
    let mut program: Vec<_> = BufReader::new(fs::File::open(input_path)?)
        .split(b'\n')
        .flatten()
        .collect();

    let mut register = [init_value, 0, 0, 0];
    let mut index = 0;

    while let Some(instruction) = program.get(index) {
        println!(
            "-> {index:2} {} [{register:?}]",
            String::from_utf8_lossy(instruction)
        );
        let (cmd, arguments) = parse_instruction(instruction);
        match (cmd, arguments.as_slice()) {
            (b"cpy", [Argument::Register(r1), Argument::Register(r2)]) => {
                register[*r2] = register[*r1];
            }
            (b"cpy", [Argument::Integer(i1), Argument::Register(r1)]) => {
                register[*r1] = *i1;
            }
            (b"inc", [Argument::Register(r1)]) => {
                register[*r1] += 1;
            }
            (b"dec", [Argument::Register(r1)]) => {
                register[*r1] -= 1;
            }
            (b"jnz", [Argument::Register(r1), Argument::Integer(i1)]) => {
                if register[*r1] != 0 {
                    index = if i1.is_negative() {
                        index.saturating_sub(i1.abs() as usize)
                    } else {
                        index.saturating_add(*i1 as usize)
                    };
                    continue;
                }
            }
            (b"jnz", [Argument::Integer(i1), Argument::Register(r1)]) => {
                if *i1 != 0 {
                    let i1 = register[*r1];
                    index = if i1.is_negative() {
                        index.saturating_sub(i1.abs() as usize)
                    } else {
                        index.saturating_add(i1 as usize)
                    };
                    continue;
                }
            }
            (b"tgl", [Argument::Register(r1)]) => {
                let i1 = register[*r1];

                let index = if i1.is_negative() {
                    index.saturating_sub(i1.abs() as usize)
                } else {
                    index.saturating_add(i1 as usize)
                };
                if let Some(instruction) = program.get_mut(index) {
                    match &instruction[..3] {
                        b"inc" => instruction[..3].copy_from_slice(b"dec"),
                        b"dec" | b"tgl" => instruction[..3].copy_from_slice(b"inc"),
                        b"jnz" => instruction[..3].copy_from_slice(b"cpy"),
                        b"cpy" => instruction[..3].copy_from_slice(b"jnz"),
                        _ => panic!("PANIC2: {}", String::from_utf8_lossy(instruction)),
                    }
                }
            }
            _ => {
                return Err(format!(
                    "invalid instruction: {} {cmd:?} {arguments:?}",
                    String::from_utf8_lossy(instruction)
                )
                .into());
            }
        }
        index += 1;
    }

    Ok(register[0])
}

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    run(input_path, 7)
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    // The safe doesn't open, but it does make several angry noises to express its frustration.

    // You're quite sure your logic is working correctly, so the only other thing is... you check
    // the painting again. As it turns out, colored eggs are still eggs. Now you count 12.

    // As you run the program with this new input, the prototype computer begins to overheat. You
    // wonder what's taking so long, and whether the lack of any instruction more powerful than
    // "add one" has anything to do with it. Don't bunnies usually multiply?

    // Anyway, what value should actually be sent to the safe?
    run(input_path, 12)
}
