use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashSet;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut fight: Fight = input.parse()?;
    
    while !fight.is_end() {
        // println!("{}", fight);
        fight.attack()?;
        // println!("");
    }
    writeln!(io::stdout(), "part1 answer: {}", fight.summary())?;
    Ok(())
}

impl Fight {
    fn is_end(&self) -> bool {
        for army in &self.armies {
            if army.iter().fold(0, |sum, g| sum + g.units) == 0 {
                return true;
            }
        }
        false
    }

    fn summary(&self) -> u64 {
        let mut sum = 0;
        for army in &self.armies {
            sum += army.iter().fold(0, |sum, g| sum + g.units)
        }
        sum
    }

    fn attack(&mut self) -> Result<()> {
        let attack_order = self.target_selection();
    
        for (kind, i, j) in attack_order {
            let attacker = &self.armies[kind][i];
            let mut damage = attacker.units * attacker.damage;

            let weaknesses = self.armies[1 - kind][j].weaknesses.clone();
            let hp = self.armies[1 - kind][j].hp;
            let units = self.armies[1 - kind][j].units;
            if weaknesses.contains(&attacker.attack_type) {
                damage *= 2;
            }
            // match kind {
            //     0 => println!("Immune System group {} attacks defending group {}, killing {} units", i+1, j+1, damage / hp),
            //     1 => println!("Infection group {} attacks defending group {}, killing {} units", i+1, j+1, damage / hp),
            //     _ => ()
            // }
            self.armies[1 - kind][j].units = units.saturating_sub(damage / hp);
        }
        Ok(())
    }

    fn target_selection<'a>(&'a mut self) -> Vec<(usize, usize, usize)> {
        self.armies[0].sort();
        self.armies[1].sort();

        let mut attack_order: Vec<(usize, usize, usize)> = vec![];
        for (i, j) in self.selection(&self.armies[0], &self.armies[1]) {
            attack_order.push((0, i ,j))
        }
        for (i, j) in self.selection(&self.armies[1], &self.armies[0]) {
            attack_order.push((1, i ,j))
        }

        attack_order.sort_by(|a, b| {
            let attacker_a = &self.armies[a.0][a.1];
            let attacker_b = &self.armies[b.0][b.1];
            attacker_b.initiative.cmp(&attacker_a.initiative)
        });

        attack_order
    }

    fn selection(&self, attackers: &Vec<Group>, defenders: &Vec<Group>) -> Vec<(usize, usize)> {
        let mut attack_order = vec![];
        let mut chosen = HashSet::new();
        let n = attackers.len();
        let m = defenders.len();

        for i in 0..n {
            let attacker = &attackers[i];
            let mut target = m + 1;
            if attacker.units == 0{
                continue;
            }
            let effective_power = attacker.units * attacker.damage;
            let mut defending_groups: Vec<usize> = vec![];
            let mut max_damage = 0;
            for j in 0..m {
                if chosen.contains(&j) {
                    continue;
                }
                let defender = &defenders[j];
                if defender.units == 0{
                    continue;
                }
                if attacker.immunities.contains(&defender.attack_type) {
                    continue;
                }
                let mut damage = effective_power;
                if defender.weaknesses.contains(&attacker.attack_type) {
                    damage = effective_power * 2;
                }
                if damage > max_damage {
                    max_damage = damage;
                    defending_groups.clear();
                    defending_groups.push(j);
                } else if damage == max_damage {
                    defending_groups.push(j);
                }
            }
            if defending_groups.len() == 1 {
                target = defending_groups[0];
            } else {
                let mut max_effective_power = 0;
                let mut defending_groups2 = vec![];
                for j in defending_groups {
                    let defender = &defenders[j];
                    let effective_power = defender.units * defender.damage;
                    if effective_power > max_effective_power {
                        max_effective_power = effective_power;
                        defending_groups2.clear();
                        defending_groups2.push(j);
                    } else if effective_power == max_effective_power {
                        defending_groups2.push(j);
                    }
                }
                if defending_groups2.len() == 1 {
                    target = defending_groups2[0];
                } else {
                    let mut max_initiative = 0;
                    let mut target2 = m + 1;
                    for j in defending_groups2 {
                        if defenders[j].initiative > max_initiative {
                            target2 = j;
                            max_initiative = defenders[j].initiative;
                        }
                    }
                    target = target2;
                }
            }
            if target == m + 1 {
                continue;
            }
            chosen.insert(target);
            attack_order.push((i, target));
        }
        attack_order
    }

    fn chosen_by_damage(&self, attacker: &Group, defenders: &Vec<Group>, chosen: HashSet<usize>) -> usize{
        let n = defenders.len();
        let mut target = (n + 1);
        let mut count = 0;
        let effective_power = attacker.units * attacker.damage;
        let mut damage = attacker.units * attacker.damage;
        let mut max_damage = 0;
        for i in 0..n {
            if chosen.contains(&i) {
                continue;
            }
            let defender = &defenders[i];
            if defender.units == 0 {
                continue;
            }
            if defender.weaknesses.contains(&attacker.attack_type) {
                damage = effective_power * 2;
            }
            if damage > max_damage {
                count = 1;
                target = i;
                max_damage = damage;
                continue;
            }
            if damage == max_damage { 
                count += 1
            }
        }
        target
    }

    fn chosen_by_effective_power(&self, defenders: &Vec<Group>, chosen: HashSet<usize>) -> usize{
        let n = defenders.len();
        for i in 0..n {
            if chosen.contains(&i) {
                continue;
            } 
            let defender = &defenders[i];
            if defender.units == 0 {
                continue;
            }
            return i;
        }
        n + 1
    }


    fn chosen_by_initiative(&self, defenders: &Vec<Group>, chosen: HashSet<usize>) -> usize{
        let n = defenders.len();
        let mut target = n + 1;
        let mut max_initiative = 0;
        for i in 0..n {
            if chosen.contains(&i) {
                continue;
            }
            let defender = &defenders[i];
            if defender.units == 0 {
                continue;
            }
            if defenders[i].initiative > max_initiative {
                max_initiative = defenders[i].initiative;
                target = i;
            }
        }
        target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttackType {
    Fire,
    Cold,
    Slashing,
    Radiation,
    Bludgeoning,
}

impl FromStr for AttackType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        use crate::AttackType::*;
        match s {
            "fire" => Ok(Fire),
            "cold" => Ok(Cold),
            "slashing" => Ok(Slashing),
            "radiation" => Ok(Radiation),
            "bludgeoning" => Ok(Bludgeoning),
            _ => err!("unrecognized attack type: {}", s)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    units: u64,
    hp: u64,
    damage: u64,
    attack_type: AttackType,
    immunities: Vec<AttackType>,
    weaknesses: Vec<AttackType>,
    initiative: u64,
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        let effective_power1 = self.units * self.damage;
        let effective_power2 = other.units * other.damage;
        if effective_power1 != effective_power2 {
            return effective_power2.cmp(&effective_power1)
        } else {
            return other.initiative.cmp(&self.initiative);
        }
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Fight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Immune System:")?;
        for i in 1..=self.armies[0].len() {
            let group = &self.armies[0][i-1];
            if group.units != 0 {
                writeln!(f, "Group {} contains {} units", i, group.units)?;
            }
        }
        writeln!(f, "Infection:")?;
        for i in 1..=self.armies[1].len() {
            let group = &self.armies[1][i-1];
            if group.units != 0 {
                writeln!(f, "Group {} contains {} units", i, group.units)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Group {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut units = 0;
        let mut hp = 0;
        let mut damage = 0;
        let mut initiative = 0;
        let mut attack_type = AttackType::Bludgeoning;
        let mut immunities = vec![];
        let mut weaknesses = vec![];
        let s = s.replace(" each with ", "\n")
            .replace(" (", "\n")
            .replace(") with an attack that does ", "\n")
            .replace("; ", "\n")
            .replace(" with an attack that does ", "\n")
            .replace(" at ", "\n");
        for line in s.lines() {
            if line.ends_with(" units") {
                units = line.strip_suffix(" units").unwrap().parse()?;
            } else if line.ends_with(" hit points") {
                hp = line.strip_suffix(" hit points").unwrap().parse()?;
            } else if line.ends_with(" damage") {
                let words: Vec<&str> = line.strip_suffix(" damage").unwrap().split(" ").collect();
                damage = words[0].parse()?;
                attack_type = words[1].parse()?;
            } else if line.starts_with("initiative ") {
                initiative = line.strip_prefix("initiative ").unwrap().parse()?;
            } else if line.starts_with("immune to ") {
                immunities = line.strip_prefix("immune to ").unwrap()
                    .split(", ")
                    .map(|w| w.parse())
                    .collect::<Result<Vec<AttackType>>>()?;
            } else if line.starts_with("weak to ") {
                weaknesses = line.strip_prefix("weak to ").unwrap()
                    .split(", ")
                    .map(|w| w.parse())
                    .collect::<Result<Vec<AttackType>>>()?;
            }
        }

        Ok(Self {
            units, hp, damage, attack_type, immunities, weaknesses, initiative
        })
    }
}

#[derive(Debug)]
struct Fight {
    armies: Vec<Vec<Group>>,
}

impl FromStr for Fight {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s: Vec<&str> = s.trim()
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.len() != 0).collect();
        let mut is_immune = true;
        let mut fight = Fight { armies: vec![vec![], vec![]] };
        for line in s {
            if line == "Immune System:" {
                is_immune = true;
                continue;
            }
            if line == "Infection:" {
                is_immune = false;
                continue;
            }
            if is_immune {
                fight.armies[0].push(line.parse()?);
            } else {
                fight.armies[1].push(line.parse()?);
            }
        }
        Ok(fight)
    }

}