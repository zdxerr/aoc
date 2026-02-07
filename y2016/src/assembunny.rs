use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

type Register = usize;
type Integer = i64;

#[derive(Debug)]
enum Instruction {
    Inc(Register),
    Dec(Register),
    CpyRegister(Register, Register),
    CpyInteger(Integer, Register),
    JnzRegisterToRegister(Register, Register),
    JnzRegisterToInteger(Register, Integer),
    JnzIntegerToRegister(Integer, Register),
    JnzIntegerToInteger(Integer, Integer),
    Tgl(Register),
    Out(Register),
}

macro_rules! num {
    ($bytes:expr, $instruction:expr) => {
        std::str::from_utf8($bytes)?.trim().parse().map_err(|e| {
            format!(
                "invalid instruction: {} ({e})",
                String::from_utf8_lossy($instruction),
            )
        })
    };
}

impl Instruction {
    fn parse(bytes: Vec<u8>) -> Result<Instruction, Box<dyn std::error::Error>> {
        let mut parts = bytes.split(|&c| c == b' ');
        Ok(match (parts.next(), parts.next(), parts.next()) {
            (Some(b"inc"), Some([byte]), None) if (b'a'..=b'd').contains(byte) => {
                Instruction::Inc((byte - b'a') as Register)
            }
            (Some(b"dec"), Some([byte]), None) if (b'a'..=b'd').contains(byte) => {
                Instruction::Dec((byte - b'a') as Register)
            }
            (Some(b"cpy"), Some([byte0]), Some([byte1]))
                if (b'a'..=b'd').contains(byte0) && (b'a'..=b'd').contains(byte1) =>
            {
                Instruction::CpyRegister((byte0 - b'a') as Register, (byte1 - b'a') as Register)
            }
            (Some(b"cpy"), Some(bytes0), Some([byte1])) if (b'a'..=b'd').contains(byte1) => {
                Instruction::CpyInteger(num!(bytes0, &bytes)?, (byte1 - b'a') as Register)
            }
            (Some(b"jnz"), Some([byte0]), Some([byte1]))
                if (b'a'..=b'd').contains(byte0) && (b'a'..=b'd').contains(byte1) =>
            {
                Instruction::JnzRegisterToRegister(
                    (byte0 - b'a') as Register,
                    (byte1 - b'a') as Register,
                )
            }
            (Some(b"jnz"), Some([byte0]), Some(bytes1)) if (b'a'..=b'd').contains(byte0) => {
                Instruction::JnzRegisterToInteger((byte0 - b'a') as Register, num!(bytes1, &bytes)?)
            }
            (Some(b"jnz"), Some(bytes0), Some([byte1])) if (b'a'..=b'd').contains(byte1) => {
                Instruction::JnzIntegerToRegister(num!(bytes0, &bytes)?, (byte1 - b'a') as Register)
            }
            (Some(b"jnz"), Some(bytes0), Some(bytes1)) => {
                Instruction::JnzIntegerToInteger(num!(bytes0, &bytes)?, num!(bytes1, &bytes)?)
            }
            (Some(b"tgl"), Some([byte]), None) if (b'a'..=b'd').contains(byte) => {
                Instruction::Tgl((byte - b'a') as Register)
            }
            (Some(b"out"), Some([byte]), None) if (b'a'..=b'd').contains(byte) => {
                Instruction::Out((byte - b'a') as Register)
            }
            _ => Err(format!(
                "invalid instruction: {}",
                String::from_utf8_lossy(&bytes)
            ))?,
        })
    }
}

// multiplication sequence can be optimized to improve performance
fn optimize_multiplication(
    program: &[Instruction],
    index: usize,
) -> Option<(&usize, &usize, usize)> {
    if let (
        Some(Instruction::Inc(_)),
        Some(Instruction::Dec(a)),
        Some(Instruction::JnzRegisterToInteger(a1, -2)),
        Some(Instruction::Dec(b)),
        Some(Instruction::JnzRegisterToInteger(b1, -5)),
    ) = (
        program.get(index),
        program.get(index + 1),
        program.get(index + 2),
        program.get(index + 3),
        program.get(index + 4),
    ) && a == a1
        && b == b1
    {
        Some((a, b, 5))
    } else {
        None
    }
}

pub fn run<F>(
    input_path: &PathBuf,
    register: &mut [i64; 4],
    mut callback: F,
) -> Result<Integer, Box<dyn std::error::Error>>
where
    F: FnMut(Integer) -> bool,
{
    let mut program: Vec<_> = BufReader::new(fs::File::open(input_path)?)
        .split(b'\n')
        .flatten()
        .map(Instruction::parse)
        .collect::<Result<Vec<Instruction>, Box<dyn std::error::Error>>>()?;

    let mut index = 0;
    while let Some(instruction) = program.get(index) {
        match instruction {
            Instruction::Inc(r1) => {
                if let Some((ra, rb, skip)) = optimize_multiplication(&program, index) {
                    register[*r1] += register[*ra] * register[*rb];
                    index += skip;
                    continue;
                }
                register[*r1] += 1;
            }
            Instruction::Dec(r1) => {
                register[*r1] -= 1;
            }
            Instruction::CpyRegister(r1, r2) => {
                register[*r2] = register[*r1];
            }
            Instruction::CpyInteger(i1, r1) => {
                register[*r1] = *i1;
            }
            Instruction::JnzRegisterToRegister(r1, r2) => {
                if register[*r1] != 0 {
                    let i2 = register[*r2 as usize];
                    index = if i2.is_negative() {
                        index.saturating_sub(i2.abs() as usize)
                    } else {
                        index.saturating_add(i2 as usize)
                    };
                    continue;
                }
            }
            Instruction::JnzRegisterToInteger(r1, i2) => {
                if register[*r1] != 0 {
                    index = if i2.is_negative() {
                        index.saturating_sub(i2.abs() as usize)
                    } else {
                        index.saturating_add(*i2 as usize)
                    };
                    continue;
                }
            }
            Instruction::JnzIntegerToRegister(i1, r2) => {
                if *i1 != 0 {
                    let i2 = register[*r2 as usize];
                    index = if i2.is_negative() {
                        index.saturating_sub(i2.abs() as usize)
                    } else {
                        index.saturating_add(i2 as usize)
                    };
                    continue;
                }
            }
            Instruction::JnzIntegerToInteger(i1, i2) => {
                if *i1 != 0 {
                    index = if i2.is_negative() {
                        index.saturating_sub(i2.abs() as usize)
                    } else {
                        index.saturating_add(*i2 as usize)
                    };
                    continue;
                }
            }
            Instruction::Tgl(r1) => {
                let i1 = register[*r1];
                let index = if i1.is_negative() {
                    index.saturating_sub(i1.abs() as usize)
                } else {
                    index.saturating_add(i1 as usize)
                };

                if let Some(instruction) = program.get_mut(index) {
                    *instruction = match instruction {
                        Instruction::Inc(r1) => Instruction::Dec(*r1),
                        Instruction::Dec(r1) | Instruction::Tgl(r1) => Instruction::Inc(*r1),
                        Instruction::CpyRegister(r1, r2) => {
                            Instruction::JnzRegisterToRegister(*r1, *r2)
                        }
                        Instruction::CpyInteger(i1, r2) => {
                            Instruction::JnzIntegerToRegister(*i1, *r2)
                        }
                        Instruction::JnzIntegerToRegister(i1, r2) => {
                            Instruction::CpyInteger(*i1, *r2)
                        }
                        _ => Err(format!(
                            "unexpected instruction toggled [{index}]: {instruction:?}"
                        ))?,
                    }
                }
            }
            Instruction::Out(r1) => {
                if !callback(register[*r1]) {
                    break;
                }
            }
        }
        index += 1;
    }

    Ok(register[0])
}
