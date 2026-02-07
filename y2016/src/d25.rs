use crate::assembunny::run;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    for i in 0.. {
        let mut register = [i, 0, 0, 0];
        // let mut output = Vec::new();
        let mut expected_out = 0;
        let mut found = 0;
        run(input_path, &mut register, |out| {
            if out == expected_out {
                found += 1;
                if found > 9 {
                    return false;
                }
                expected_out ^= 1;
                return true;
            }
            false
        })?;

        if found > 9 {
            return Ok(i);
        }
    }
    Err("no solution found".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}
