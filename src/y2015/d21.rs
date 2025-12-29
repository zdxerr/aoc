use std::fs;
use std::path::PathBuf;

type Item = (u64, u64, u64);

const WEAPONS: [Item; 5] = [
    (8, 4, 0),  // Dagger
    (10, 5, 0), // Shortsword
    (25, 6, 0), // Warhammer
    (40, 7, 0), // Longsword
    (74, 8, 0), // Greataxe
];

const ARMOR: [Item; 6] = [
    (0, 0, 0),
    (13, 0, 1),  // Leather
    (31, 0, 2),  // Chainmail
    (53, 0, 3),  // Spintmail
    (75, 0, 4),  // Bandemail
    (102, 0, 5), // Platemail
];

const RINGS: [Item; 7] = [
    (0, 0, 0),
    (25, 1, 0),  // Damage +1
    (50, 2, 0),  // Damage +2
    (100, 3, 0), // Damage +3
    (20, 0, 1),  // Defense +1
    (40, 0, 2),  // Defense +2
    (80, 0, 3),  // Defense +3
];

pub fn part1(input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    let mut stats = content
        .lines()
        .map(|line| line.split_once(": "))
        .flatten()
        .map(|(_, num)| num.parse::<u64>().unwrap());

    let (boss_hit_points, boss_damage, boss_armor) =
        if let (Some(hit_points), Some(damage), Some(armor)) =
            (stats.next(), stats.next(), stats.next())
        {
            (hit_points, damage, armor)
        } else {
            panic!("unable to parse input");
        };

    let mut min_cost: Option<u64> = None;

    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring1 in RINGS {
                for ring2 in RINGS {
                    if ring1 == ring2 && ring2 != RINGS[0] {
                        continue;
                    }
                    let player_cost: u64 = [weapon.0, armor.0, ring1.0, ring2.0].iter().sum();
                    let player_damage: u64 = [weapon.1, armor.1, ring1.1, ring2.1].iter().sum();
                    let player_armor: u64 = [weapon.2, armor.2, ring1.2, ring2.2].iter().sum();
                    // println!("{cost} {damage} {armor}");
                    let a = player_damage.checked_sub(boss_armor).unwrap_or(1).max(1);
                    let b = boss_damage.checked_sub(player_armor).unwrap_or(1).max(1);

                    let player_rounds = 100_u64.div_ceil(b);
                    let boss_rounds = boss_hit_points.div_ceil(a);

                    // println!(
                    //     "cost: {player_cost}, player rounds: {player_rounds}, boss rounds: {boss_rounds} {} ({a}, {b})",
                    //     player_rounds >= boss_rounds
                    // );

                    if player_rounds >= boss_rounds {
                        if let Some(ref mut min_cost) = min_cost {
                            if *min_cost > player_cost {
                                // println!(
                                //     "######################## {weapon:?},{armor:?},{ring1:?},{ring2:?} {min_cost:?} {player_cost}"
                                // );
                                *min_cost = player_cost;
                            }
                        } else {
                            min_cost = Some(player_cost)
                        }
                    }
                }
            }
        }
    }
    min_cost.ok_or("no winning combination found".into())
    // Err("not implemented".into())
}

pub fn part2(_input_path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Err("not implemented".into())
}
