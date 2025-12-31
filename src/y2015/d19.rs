use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub fn part1(input_path: &PathBuf) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut lines = content.lines().rev();
    let molecule = lines.next().ok_or("unable to read molecule")?;

    let mut molecules: HashSet<String> = HashSet::new();

    for (from, to) in lines.map(|line| line.split_once(" => ")).flatten() {
        let mut start = 0;
        while let Some(mut pos) = molecule[start..].find(&from) {
            pos += start;
            let end = pos + from.len();
            molecules.insert(format!("{}{}{}", &molecule[..pos], to, &molecule[end..]));
            start = end;
        }
    }
    Ok(molecules.len())
}

pub fn part2(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    // There are only two types of productions:
    // e => XX and X => XX (X is not Rn, Y, or Ar)
    // X => X Rn X Ar | X Rn X Y X Ar | X Rn X Y X Y X Ar
    //
    // You can think of Rn Y Ar as the characters ( , ):
    // X => X(X) | X(X,X) | X(X,X,X)
    //
    // Repeatedly applying X => XX until you arrive at a single token takes count(tokens) - 1 steps.
    //
    // Applying X => X(X) is similar to X => XX, except you get the () for free.
    //
    // You can generalize to X => X(X,X) by noting that each , reduces the length by two (,X).
    // The new formula is count(tokens) - count("(" or ")") - 2*count(",") - 1.

    let content = fs::read_to_string(input_path)?;
    let mut lines = content.lines().rev();
    let molecule = lines.next().ok_or("unable to read molecule")?;

    // Transform brackets and commas.
    let molecule = molecule
        .replace("Rn", "(")
        .replace("Ar", ")")
        .replace("Y", ",");
    // Ignore lower case latters.
    let molecule: String = molecule
        .chars()
        .filter(|c| c.is_ascii_uppercase() || c.is_ascii_punctuation())
        .collect();

    // Count total elements, brackets and commas.
    let (elements, brackets, commas) =
        molecule
            .chars()
            .fold((0, 0, 0), |(mut elements, mut brackets, mut commas), c| {
                elements += 1;
                match c {
                    '(' | ')' => {
                        brackets += 1;
                    }
                    ',' => {
                        commas += 1;
                    }
                    _ => {}
                }
                (elements, brackets, commas)
            });
    let sum = elements - brackets - 2 * commas - 1;
    Ok(sum)
}
