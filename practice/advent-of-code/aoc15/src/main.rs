use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let caves: Caves = input.parse()?;

    writeln!(io::stdout(), "part1 outcome: {}", caves.clone().outcome(false)?)?;

    for power in 4..100 {
        let mut caves = caves.clone();
        caves.set_elf_attack_power(power);

        let initial_elves = caves.remaining_elves();
        let outcome = caves.outcome(true)?;
        if initial_elves == caves.remaining_elves() {
            writeln!(
                io::stdout(),
                "part2, eveles at power {}, outcome: {}",
                3 + power, outcome,
            )?;
            break;
        }
    }

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Caves {
    grid: BTreeMap<Coordinate, Cell>,
    units: BTreeMap<Coordinate, Unit>,
    max: Coordinate,
}

#[derive(Debug, Clone)]
enum Cell {
    Wall,
    Open,
}

impl Caves {
    fn outcome(&mut self, stop_when_elf_killed: bool) -> Result<usize> {
        const LIMIT: usize = 500;

        for i in 0..LIMIT {
            let moved = self.step(stop_when_elf_killed);
            if !moved {
                let hp = self.hp();
                return Ok(i * hp)
            }
        }
        err!("no out come after {} iterations", LIMIT)
    }

    fn remaining_elves(&self) -> usize {
        self.units.values().filter(|u| u.is_elf()).count()
    }

    fn set_elf_attack_power(&mut self, power: usize) {
        for unit in self.units.values_mut() {
            if unit.is_elf() {
                unit.attack += power;
            }
        }
    }

    fn hp(&self) -> usize {
        self.units.values().map(|u| u.hp).sum()
    }

    fn step(&mut self, stop_when_elf_killed: bool) -> bool {
        let mut any_move = false;
        let unit_coordinates: Vec<_> = self.units.keys().cloned().collect();
        for c in unit_coordinates.into_iter() {
            if !self.units.contains_key(&c) {
                continue;
            }
            if !self.any_enemies(c) {
                return false;
            }
            if let Some(attack) = self.best_attack_unit(c) {
                if let Some(kind) = self.attack(c, attack) {
                    match kind {
                        UnitKind::Elf => if stop_when_elf_killed { return false },
                        UnitKind::Goblin => {}
                    }
                }
                any_move = true;
                continue;
            }

            let nextc = match self.next_step(c) {
                None => continue,
                Some(nextc) => nextc,
            };
            any_move = true;
            
            let unit = self.units.remove(&c).unwrap();
            self.units.insert(nextc, unit);
            if let Some(attack) = self.best_attack_unit(nextc) {
                if let Some(kind) = self.attack(nextc, attack) {
                    match kind {
                        UnitKind::Elf => if stop_when_elf_killed { return false },
                        UnitKind::Goblin => {}
                    }
                }
            }
        }
        any_move
    }
    
    fn next_step(&self, unit: Coordinate) -> Option<Coordinate> {
        self.nearest_target(unit).and_then(|t| self.nearest_step(unit, t))
    }
    
    fn nearest_step(&self, unit: Coordinate, target: Coordinate) -> Option<Coordinate> {
        let dists = self.distances(target);
        self.neighbors(unit)
            .filter_map(|c| dists.get(&c).map(|dist| (c, dist)))
            .min_by_key(|&(_, dist)| dist)
            .map(|(c, _)| c)
    }

    fn nearest_target(&self, unit: Coordinate) -> Option<Coordinate> {
        let dists = self.distances(unit);
        self.targets(unit)
            .into_iter()
            .filter_map(|c| dists.get(&c).map(|dist| (c, dist)))
            .min_by_key(|&(_, dist)| dist)
            .map(|(c,_)| c)
    }

    fn distances(&self, origin: Coordinate) -> BTreeMap<Coordinate, usize> {
        let mut d = BTreeMap::new();
        d.insert(origin, 0);

        let mut todo = VecDeque::new();
        todo.push_front(origin);
        let mut todo_set = BTreeSet::new();
        let mut visited = BTreeSet::new();
        while let Some(c) = todo.pop_front() {
            visited.insert(c);
            todo_set.remove(&c);
            for neighbor in self.neighbors(c) {
                if visited.contains(&neighbor) {
                    continue;
                }
                if !todo_set.contains(&neighbor) {
                    todo.push_back(neighbor);
                    todo_set.insert(neighbor);
                }

                let candidate_dist = 1 + *d.get(&c).unwrap_or(&0);
                if !d.contains_key(&neighbor) || candidate_dist < d[&neighbor] {
                    d.insert(neighbor, candidate_dist);
                }
            }
        }
        d
    }

    fn targets(&self, origin: Coordinate) -> BTreeSet<Coordinate> {
        let unit = &self.units[&origin];
        let mut targets = BTreeSet::new();
        for (&c, candidate) in &self.units {
            if unit.is_enemy(candidate) {
                targets.extend(self.neighbors(c));
            }
        }
        targets
    }

    fn any_enemies(&self, unit: Coordinate) -> bool {
        for candidate in self.units.values() {
            if self.units[&unit].is_enemy(candidate) {
                return true;
            }
        }
        false
    }

    fn attack(&mut self, attacker: Coordinate, victim: Coordinate) -> Option<UnitKind>{
        let power = self.units[&attacker].attack;
        if self.units.get_mut(&victim).unwrap().absorb(power) {
            let dead_kind = Some(self.units.get(&victim).unwrap().kind);
            self.units.remove(&victim);
            return dead_kind;
        }
        None
    }

    fn best_attack_unit(&self, c: Coordinate) -> Option<Coordinate> {
        let unit = &self.units[&c];
        c.neighbors(self.max)
            .into_iter()
            .filter(|c| self.units.contains_key(c))
            .filter(|c| unit.is_enemy(&self.units[c]))
            .min_by_key(|c| (self.units[c].hp, *c))
    }

    fn neighbors<'a>(&'a self, origin: Coordinate) -> impl Iterator<Item=Coordinate> + 'a {
        origin
            .neighbors(self.max)
            .into_iter()
            .filter(move |&c| self.is_open(c))
    }

    fn is_open(&self, c: Coordinate) -> bool {
        !self.units.contains_key(&c) && self.grid[&c].is_open()
    }
}

impl Cell {
    fn is_open(&self) -> bool {
        match *self {
            Cell::Open => true,
            Cell::Wall => false,
        }
    }
}

impl FromStr for Caves {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() {
            return err!("only ASCII caves are supported");
        }
        let mut caves = Caves::default();
        caves.max.x = s.lines().next().unwrap_or("").len() - 1;
        caves.max.y = s.lines().count() - 1;
        for (y, line) in s.lines().enumerate() {
            for (x, _) in line.char_indices() {
                let c = Coordinate { x, y };
                let cell = &line[x..x+1];
                if ["E", "G"].contains(&cell) {
                    let unit  = cell.parse()?;
                    caves.grid.insert(c, Cell::Open);
                    caves.units.insert(c, unit);
                } else {
                    caves.grid.insert(c, cell.parse()?);
                }
            }
        }

        Ok(caves)
    }
}

impl FromStr for Cell {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        match s.as_bytes().get(0) {
            None => err!("cannot deserialize empty string into cell"),
            Some(&b'#') => Ok(Cell::Wall),
            Some(&b'.') => Ok(Cell::Open),
            Some(&b) => err!("unrecognized cell: 0x{:x}", b),
        }
    }
}

impl fmt::Display for Caves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut units = vec![];
        for (c, cell) in &self.grid {
            if let Some(ref unit) = self.units.get(c) {
                write!(f, "{}", unit)?;
                units.push(format!("{:?}", unit));
            } else {
                write!(f, "{}", cell)?;
            }
            if c.x == self.max.x {
                write!(f, "    {}", units.join(", "))?;
                units.clear();
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Open => write!(f, "."),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize, 
    y: usize,
}

impl Coordinate {
    fn with_x(self, x: usize) -> Coordinate {
        Coordinate { x, ..self }
    }

    fn with_y(self, y: usize) -> Coordinate {
        Coordinate { y, ..self }
    }

    fn distance(&self, other: Coordinate) -> usize {
        let x = (self.x as isize - other.x as isize).abs();
        let y = (self.y as isize - other.y as isize).abs();
        (x + y) as usize
    }

    fn neighbors(self, max: Coordinate) -> Vec<Coordinate> {
        assert!(self <= max, "{:?} should be <= than the max {:?}", self, max);
        let mut coords = vec![];
        if self.y > 0 {
            coords.push(self.with_y(self.y - 1));
        }
        if self.x > 0 {
            coords.push(self.with_x(self.x - 1));
        }
        if self.x + 1 <= max.x {
            coords.push(self.with_x(self.x + 1));
        }
        if self.y + 1 <= max.y {
            coords.push(self.with_y(self.y + 1));
        }
        coords
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.y, self.x).cmp(&(other.y, other.x)))
    }
}

#[derive(Clone)]
struct Unit {
    attack: usize,
    hp: usize,
    kind: UnitKind,
}

#[derive(Clone, Copy, Debug)]
enum UnitKind {
    Elf,
    Goblin,
}

impl Unit {
    fn is_enemy(&self, other: &Unit) -> bool {
        use crate::UnitKind::*;

        match (self.kind, other.kind) {
            (Elf, Goblin) => true,
            (Goblin, Elf) => true,
            _ => false
        }
    }

    fn is_elf(&self) -> bool {
        match self.kind {
            UnitKind::Elf => true,
            UnitKind::Goblin => false,
        }
    }
    
    fn absorb(&mut self, power: usize) -> bool {
        self.hp = self.hp.saturating_sub(power);
        self.is_dead()
    }

    fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

impl FromStr for Unit {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let kind = match s.as_bytes().get(0) {
            None => return err!("cannot deserialize empty string into unit"),
            Some(&b'E') => UnitKind::Elf,
            Some(&b'G') => UnitKind::Goblin,
            Some(&b) => return err!("unrecognized unit kind: 0x{:x}", b),
        };
        Ok(Self{ attack: 3, hp: 200, kind})
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            UnitKind::Elf => write!(f, "E"),
            UnitKind::Goblin => write!(f, "G"),
        }
    }
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            UnitKind::Elf => write!(f, "E({})", self.hp),
            UnitKind::Goblin => write!(f, "G({})", self.hp),
        }
    }
}
