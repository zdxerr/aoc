use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

enum Command {
    CPY(i32, usize),
    INC(usize),
    DEC(usize),
    JNZ(usize, usize),
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
    let content = fs::read_to_string(input_path)?;
    let reader = BufReader::new(fs::File::open(input_path)?);

    let mut pos = 0;
    // let mut a = None;

    let program: Vec<Command> = reader
        .split(b'\n')
        .flatten()
        .map(|line| match &line[0..3] {
            b"cpy" => Command::CPY(5, 5),
            b"inc" => Command::INC(5),
            b"dec" => Command::DEC(5),
            b"jnz" => Command::JNZ(5, 5),
            _ => panic!("PANIC"),
        })
        .collect();

    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
