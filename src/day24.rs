use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Group {
    clan: String,
    units: usize,
    hp_per_unit: usize,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    attack_damage: usize,
    attack_type: String,
}

impl Group {
    pub fn power(&self) -> usize {
        self.units * self.attack_damage
    }

    pub fn damage_to(&self, target: &Group) -> usize {
        let rate = if target.weaknesses.contains(&self.attack_type) {
            2
        } else if target.immunities.contains(&self.attack_type) {
            0
        } else {
            1
        };

        rate * self.power()
    }
}

impl FromStr for Group {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(concat!(
            r"(\d+) units each with (\d+) hit points (\((.*)\) )?",
            r"with an attack that does (\d+) (\w+) damage at initiative (\d+)"
        ))
        .unwrap();
        let caps = re.captures(s).ok_or("Group string dose not match")?;
        let mut weaknesses = vec![];
        let mut immunities = vec![];

        const WEAKNESS_PREFIX: &str = "weak to ";
        const IMMUNITY_PREFIX: &str = "immune to ";

        if let Some(attr_str) = caps.get(4) {
            for attr in attr_str.as_str().split(";").map(str::trim) {
                if attr.starts_with(WEAKNESS_PREFIX) {
                    weaknesses = attr[WEAKNESS_PREFIX.len()..]
                        .split(",")
                        .map(|s| s.trim().to_owned())
                        .collect();
                } else if attr.starts_with(IMMUNITY_PREFIX) {
                    immunities = attr[IMMUNITY_PREFIX.len()..]
                        .split(",")
                        .map(|s| s.trim().to_owned())
                        .collect();
                }
            }
        }

        Ok(Group {
            clan: String::new(),
            units: caps[1].parse()?,
            hp_per_unit: caps[2].parse()?,
            initiative: caps[7].parse()?,
            weaknesses,
            immunities,
            attack_damage: caps[5].parse()?,
            attack_type: caps[6].parse()?,
        })
    }
}

pub fn battle(mut groups: Vec<Group>) -> usize {
    loop {
        groups.sort_by_key(|x| (Reverse(x.power()), Reverse(x.initiative)));
        let mut targets = vec![None; groups.len()];
        for i in 0..groups.len() {
            let mut max_damage = 0;
            for j in 0..groups.len() {
                if groups[i].clan == groups[j].clan
                    || targets.contains(&Some(j))
                    || groups[j].units == 0
                {
                    continue;
                }

                let damage = groups[i].damage_to(&groups[j]);
                if damage > max_damage {
                    max_damage = damage;
                    targets[i] = Some(j);
                }
            }
        }

        let mut attackers: Vec<_> = (0..groups.len()).collect();
        attackers.sort_by_key(|&i| Reverse(groups[i].initiative));
        for attacker_idx in attackers {
            if groups[attacker_idx].units == 0 {
                continue;
            }

            if let Some(defender_idx) = targets[attacker_idx] {
                let damage = groups[attacker_idx].damage_to(&groups[defender_idx]);
                groups[defender_idx].units = groups[defender_idx]
                    .units
                    .saturating_sub(damage / groups[defender_idx].hp_per_unit);
            }
        }

        let mut stats = HashMap::new();
        groups
            .iter()
            .filter(|group| group.units > 0)
            .for_each(|group| {
                stats
                    .entry(group.clan.clone())
                    .and_modify(|c| *c += group.units)
                    .or_insert(group.units);
            });

        if stats.get("Immune System").is_none() {
            return stats["Infection"];
        } else if stats.get("Infection").is_none() {
            return stats["Immune System"];
        }
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Group> {
    let mut groups = vec![];
    let armies_iter = input.split("\n\n");

    for army in armies_iter {
        let mut iter = army.lines();
        let clan = iter.next().unwrap().trim_end_matches(":");
        let clan_groups = iter.map(move |line| {
            let mut group: Group = line.parse().unwrap();
            group.clan = clan.to_owned();
            group
        });
        groups.extend(clan_groups);
    }

    groups
}

#[aoc(day24, part1)]
pub fn solve_part1(groups: &Vec<Group>) -> usize {
    battle(groups.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let groups = input_generator(input);
        assert_eq!(battle(groups), 5216);
    }
}
