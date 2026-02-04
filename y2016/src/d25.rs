use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let _content = fs::read_to_string(input_path)?;
    let _reader = BufReader::new(fs::File::open(input_path)?);

    Err("not implemented".into())
}
// cpy a d
// cpy 14 c
// cpy 182 b
// inc d
// dec b
// jnz b -2
// dec c
// jnz c -5
// cpy d a d=a
// jnz 0 0 d=a+14*182
// cpy a b b=a+14*182
// cpy 0 a a=0
// cpy 2 c c=2
// jnz b 2
// jnz 1 6
// dec b
// dec c
// jnz c -4
// inc a
// jnz 1 -7
// cpy 2 b
// jnz c 2
// jnz 1 4
// dec b
// dec c
// jnz 1 -4
// jnz 0 0
// out b
// jnz a -19
// jnz 1 -21

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
