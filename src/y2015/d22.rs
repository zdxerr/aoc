use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: u16,
    boss_hp: i16,
    player_hp: i16,
    player_mana: i16,
    shield_effect: u8,
    poison_effect: u8,
    recharge_effect: u8,
}

impl State {
    /// Applies spell effects to state and returns true if the player has won.
    #[inline]
    fn apply_spell_effects(&mut self) -> bool {
        if self.shield_effect > 0 {
            self.shield_effect -= 1;
        }
        if self.poison_effect > 0 {
            self.poison_effect -= 1;
            self.boss_hp -= 3;
        }
        if self.recharge_effect > 0 {
            self.recharge_effect -= 1;
            self.player_mana += 101;
        }

        self.boss_hp <= 0
    }

    /// Applies boss attack and returns true if the wizard survives.
    #[inline]
    fn boss_turn(&mut self, mut attack: i16) -> bool {
        if self.shield_effect > 0 {
            attack = (attack - 7).max(1);
        }

        self.player_hp -= attack;
        self.player_hp > 0 && self.player_mana >= 53
    }
}

// Reverse Order for cost to use in a BinaryHeap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.boss_hp.hash(state);
        self.player_hp.hash(state);
        self.player_mana.hash(state);
        self.shield_effect.hash(state);
        self.poison_effect.hash(state);
        self.recharge_effect.hash(state);
    }
}

fn parse(input_path: &PathBuf) -> Result<(i16, i16), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;

    let mut stats = content
        .lines()
        .map(|line| line.split_once(": "))
        .flatten()
        .map(|(_, num)| num.parse::<i16>().unwrap());

    if let (Some(hit_points), Some(damage)) = (stats.next(), stats.next()) {
        Ok((hit_points, damage))
    } else {
        panic!("unable to parse input");
    }
}

fn solve(input_path: &PathBuf, hard: bool) -> Result<u16, Box<dyn std::error::Error>> {
    let (boss_hit_points, boss_damage) = parse(input_path)?;

    let mut queue: BinaryHeap<State> = BinaryHeap::with_capacity(1000);
    let mut cache: HashSet<State> = HashSet::with_capacity(1000);

    queue.push(State {
        cost: 0,
        boss_hp: boss_hit_points,
        player_hp: 50,
        player_mana: 500,
        shield_effect: 0,
        poison_effect: 0,
        recharge_effect: 0,
    });

    while let Some(mut state) = queue.pop() {
        if state.apply_spell_effects() {
            return Ok(state.cost);
        }

        if hard {
            state.player_hp -= 1;
        }

        // Magic Missile
        if state.player_mana >= 53 {
            let mut next = State {
                cost: state.cost + 53,
                boss_hp: state.boss_hp - 4,
                player_mana: state.player_mana - 53,
                ..state
            };

            if next.apply_spell_effects() {
                return Ok(next.cost);
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                queue.push(next);
            }
        }

        // Drain
        if state.player_mana >= 73 {
            let mut next = State {
                cost: state.cost + 73,
                boss_hp: state.boss_hp - 2,
                player_hp: state.player_hp + 2,
                player_mana: state.player_mana - 73,
                ..state
            };

            if next.apply_spell_effects() {
                return Ok(next.cost);
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                queue.push(next);
            }
        }

        // Shield
        if state.player_mana >= 113 && state.shield_effect == 0 {
            let mut next = State {
                cost: state.cost + 113,
                player_mana: state.player_mana - 113,
                shield_effect: 6,
                ..state
            };

            if next.apply_spell_effects() {
                return Ok(next.cost);
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                queue.push(next);
            }
        }

        // Poison
        if state.player_mana >= 173 && state.poison_effect == 0 {
            let mut next = State {
                cost: state.cost + 173,
                player_mana: state.player_mana - 173,
                poison_effect: 6,
                ..state
            };

            if next.apply_spell_effects() {
                return Ok(next.cost);
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                queue.push(next);
            }
        }

        // Recharge
        if state.player_mana >= 229 && state.recharge_effect == 0 {
            let mut next = State {
                cost: state.cost + 229,
                player_mana: state.player_mana - 229,
                recharge_effect: 5,
                ..state
            };

            if next.apply_spell_effects() {
                return Ok(next.cost);
            }
            if next.boss_turn(boss_damage) && cache.insert(next) {
                queue.push(next);
            }
        }
    }
    unreachable!();
}

pub fn part1(input_path: &PathBuf) -> Result<u16, Box<dyn std::error::Error>> {
    solve(input_path, false)
    // let (boss_hit_points, boss_damage) = parse(input_path)?;

    // let mut queue: BinaryHeap<State> = BinaryHeap::with_capacity(1000);
    // let mut cache: HashSet<State> = HashSet::with_capacity(1000);

    // queue.push(State {
    //     cost: 0,
    //     boss_hp: boss_hit_points,
    //     player_hp: 50,
    //     player_mana: 500,
    //     shield_effect: 0,
    //     poison_effect: 0,
    //     recharge_effect: 0,
    // });

    // while let Some(mut state) = queue.pop() {
    //     if state.apply_spell_effects() {
    //         return Ok(state.cost);
    //     }
    //     // Magic Missile
    //     if state.player_mana >= 53 {
    //         let mut next = State {
    //             cost: state.cost + 53,
    //             boss_hp: state.boss_hp - 4,
    //             player_mana: state.player_mana - 53,
    //             ..state
    //         };

    //         if next.apply_spell_effects() {
    //             return Ok(next.cost);
    //         }
    //         if next.boss_turn(boss_damage) && cache.insert(next) {
    //             queue.push(next);
    //         }
    //     }

    //     // Drain
    //     if state.player_mana >= 73 {
    //         let mut next = State {
    //             cost: state.cost + 73,
    //             boss_hp: state.boss_hp - 2,
    //             player_hp: state.player_hp + 2,
    //             player_mana: state.player_mana - 73,
    //             ..state
    //         };

    //         if next.apply_spell_effects() {
    //             return Ok(next.cost);
    //         }
    //         if next.boss_turn(boss_damage) && cache.insert(next) {
    //             queue.push(next);
    //         }
    //     }

    //     // Shield
    //     if state.player_mana >= 113 && state.shield_effect == 0 {
    //         let mut next = State {
    //             cost: state.cost + 113,
    //             player_mana: state.player_mana - 113,
    //             shield_effect: 6,
    //             ..state
    //         };

    //         if next.apply_spell_effects() {
    //             return Ok(next.cost);
    //         }
    //         if next.boss_turn(boss_damage) && cache.insert(next) {
    //             queue.push(next);
    //         }
    //     }

    //     // Poison
    //     if state.player_mana >= 173 && state.poison_effect == 0 {
    //         let mut next = State {
    //             cost: state.cost + 173,
    //             player_mana: state.player_mana - 173,
    //             poison_effect: 6,
    //             ..state
    //         };

    //         if next.apply_spell_effects() {
    //             return Ok(next.cost);
    //         }
    //         if next.boss_turn(boss_damage) && cache.insert(next) {
    //             queue.push(next);
    //         }
    //     }

    //     // Recharge
    //     if state.player_mana >= 229 && state.recharge_effect == 0 {
    //         let mut next = State {
    //             cost: state.cost + 229,
    //             player_mana: state.player_mana - 229,
    //             recharge_effect: 5,
    //             ..state
    //         };

    //         if next.apply_spell_effects() {
    //             return Ok(next.cost);
    //         }
    //         if next.boss_turn(boss_damage) && cache.insert(next) {
    //             queue.push(next);
    //         }
    //     }
    // }
    // unreachable!();
}

pub fn part2(input_path: &PathBuf) -> Result<u16, Box<dyn std::error::Error>> {
    solve(input_path, true)
}
