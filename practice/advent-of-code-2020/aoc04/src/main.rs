use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let passports = input
        .split("\n\n")
        .map(|l| l.parse())
        .collect::<Result<Vec<Passport>>>()?;

    part1(&passports)?;
    part2(&passports)?;
    Ok(())
}

fn part1(passports: &[Passport]) -> Result<()> {
    let count = passports.iter().filter(|p| p.is_valid()).count();
    writeln!(
        io::stdout(),
        "Part1: In your batch file, how many passports are valid? {}",
        count
    )?;
    Ok(())
}

fn part2(passports: &[Passport]) -> Result<()> {
    let count = passports.iter().filter(|p| p.is_valid_strict()).count();
    writeln!(
        io::stdout(),
        "Part2: In your batch file, how many passports are valid? {}",
        count
    )?;
    Ok(())
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
    count: usize,
}

impl Passport {
    fn new() -> Self {
        Self::default()
    }

    fn add_field(&mut self, field: &str, value: String) -> Result<()> {
        match field {
            "byr" => self.byr = byr(value),
            "iyr" => self.iyr = iyr(value),
            "eyr" => self.eyr = eyr(value),
            "hgt" => self.hgt = hgt(value),
            "hcl" => self.hcl = hcl(value),
            "ecl" => self.ecl = ecl(value),
            "pid" => self.pid = pid(value),
            "cid" => self.cid = Some(value),
            _ => return err!("This is not a  valid field: {}", field),
        }
        self.count += 1;
        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.count == 8 || self.count == 7 && self.cid.is_none()
    }

    fn is_valid_strict(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl FromStr for Passport {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut passport = Passport::new();
        let lines = s.split([' ', '\n']);
        for line in lines {
            if let Some((field, value)) = line.split_once(':') {
                passport.add_field(field, value.to_string())?;
            }
        }
        Ok(passport)
    }
}

fn byr(s: String) -> Option<String> {
    if let Ok(year) = s.parse::<usize>() {
        if (1920..=2002).contains(&year) {
            return Some(s);
        }
    }
    None
}

fn iyr(s: String) -> Option<String> {
    if let Ok(year) = s.parse::<usize>() {
        if (2010..=2020).contains(&year) {
            return Some(s);
        }
    }
    None
}

fn eyr(s: String) -> Option<String> {
    if let Ok(year) = s.parse::<usize>() {
        if (2020..=2030).contains(&year) {
            return Some(s);
        }
    }
    None
}

fn hgt(s: String) -> Option<String> {
    if let Some(h) = s.strip_suffix("cm") {
        if let Ok(h) = h.parse::<usize>() {
            if (150..=193).contains(&h) {
                return Some(s);
            }
        }
    } else if let Some(h) = s.strip_suffix("in") {
        if let Ok(h) = h.parse::<usize>() {
            if (59..76).contains(&h) {
                return Some(s);
            }
        }
    }
    None
}

fn hcl(s: String) -> Option<String> {
    if let Some(c) = s.strip_prefix('#') {
        if s.len() == 7
            && c.chars()
                .all(|b| b.is_numeric() | (b'a'..=b'f').any(|b1| b1 == b as u8))
        {
            return Some(s);
        }
    }
    None
}

fn ecl(s: String) -> Option<String> {
    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.as_str()) {
        return Some(s);
    }
    None
}

fn pid(s: String) -> Option<String> {
    if s.len() == 9 && s.chars().all(|b| b.is_numeric()) {
        return Some(s);
    }
    None
}
