use crate::assembunny::run;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    for i in 0..5000 {
        let mut register = [i, 0, 0, 0];
        let mut output = Vec::new();
        run(input_path, &mut register, &mut output)?;

        println!(
            "{i:03} {register:?}    {output:?}  {:?}",
            output == [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
        );
        if output == [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1] {
            break;
        }
    }
    Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}
