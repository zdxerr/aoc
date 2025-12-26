use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Ingredient(i64, i64, i64, i64, i64);

fn parse(input_path: &PathBuf) -> Result<Vec<Ingredient>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    Ok(content
        .lines()
        .map(|line| {
            let mut splitted = line.split([' ', ',']);
            Ingredient(
                splitted.nth(2).unwrap().parse().unwrap(),
                splitted.nth(2).unwrap().parse().unwrap(),
                splitted.nth(2).unwrap().parse().unwrap(),
                splitted.nth(2).unwrap().parse().unwrap(),
                splitted.nth(2).unwrap().parse().unwrap(),
            )
        })
        .collect())
}

pub fn part1(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let ingredients = parse(input_path)?;

    fn next(ingredients: &[Ingredient], idx: usize, spoons: &mut [i64], left: i64) -> i64 {
        if idx == ingredients.len() {
            return spoons
                .iter()
                .zip(ingredients)
                .map(|(spoon, ingredient)| *spoon * ingredient.0)
                .sum::<i64>()
                .max(0)
                * spoons
                    .iter()
                    .zip(ingredients)
                    .map(|(spoon, ingredient)| spoon * ingredient.1)
                    .sum::<i64>()
                    .max(0)
                * spoons
                    .iter()
                    .zip(ingredients)
                    .map(|(spoon, ingredient)| spoon * ingredient.2)
                    .sum::<i64>()
                    .max(0)
                * spoons
                    .iter()
                    .zip(ingredients)
                    .map(|(spoon, ingredient)| spoon * ingredient.3)
                    .sum::<i64>()
                    .max(0);
        }

        (0..=left)
            .map(|spoon| {
                spoons[idx] = spoon;
                next(ingredients, idx + 1, spoons, left - spoon)
            })
            .max()
            .unwrap()
    }

    Ok(next(&ingredients, 0, &mut vec![0; ingredients.len()], 100))
}

pub fn part2(input_path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    let ingredients = parse(input_path)?;

    fn next(ingredients: &[Ingredient], idx: usize, spoons: &mut [i64], left: i64) -> i64 {
        if idx == ingredients.len() {
            return if spoons
                .iter()
                .zip(ingredients)
                .map(|(spoon, ingredient)| *spoon * ingredient.4)
                .sum::<i64>()
                == 500
            {
                spoons
                    .iter()
                    .zip(ingredients)
                    .map(|(spoon, ingredient)| *spoon * ingredient.0)
                    .sum::<i64>()
                    .max(0)
                    * spoons
                        .iter()
                        .zip(ingredients)
                        .map(|(spoon, ingredient)| spoon * ingredient.1)
                        .sum::<i64>()
                        .max(0)
                    * spoons
                        .iter()
                        .zip(ingredients)
                        .map(|(spoon, ingredient)| spoon * ingredient.2)
                        .sum::<i64>()
                        .max(0)
                    * spoons
                        .iter()
                        .zip(ingredients)
                        .map(|(spoon, ingredient)| spoon * ingredient.3)
                        .sum::<i64>()
                        .max(0)
            } else {
                0
            };
        }

        (0..=left)
            .map(|spoon| {
                spoons[idx] = spoon;
                next(ingredients, idx + 1, spoons, left - spoon)
            })
            .max()
            .unwrap()
    }

    Ok(next(&ingredients, 0, &mut vec![0; ingredients.len()], 100))
}
