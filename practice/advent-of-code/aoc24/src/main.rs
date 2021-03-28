use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let armies: Armies = input.parse()?;

    println!("{:?}", armies);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
struct Group {
    units: u32,
    hp: i32,
    damage: u32,
    attack_type: AttackType,
    immunities: Vec<AttackType>,
    weaknesses: Vec<AttackType>,
    initiative: u32,
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
            .replace("with an attack that does ", "\n")
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
struct Armies {
    immune: Vec<Group>,
    infection: Vec<Group>,
}

impl FromStr for Armies {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s: Vec<&str> = s.trim()
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.len() != 0).collect();
        let mut is_immune = true;
        let mut immune: Vec<Group> = vec![];
        let mut infection: Vec<Group> = vec![];
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
                immune.push(line.parse()?);
            } else {
                infection.push(line.parse()?);
            }
        }
        Ok(Self { immune, infection })
    }

}