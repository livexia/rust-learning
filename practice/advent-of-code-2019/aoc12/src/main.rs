use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = i64;
type Pos = (Coord, Coord, Coord);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let moons = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Moon>>>()?;

    part1(moons.clone(), 1000)?;
    part2(moons.clone())?;
    Ok(())
}

fn part1(mut moons: Vec<Moon>, steps: usize) -> Result<Coord> {
    let start = Instant::now();

    let l = moons.len();
    for _ in 0..steps {
        for i in 0..l {
            for j in 0..l {
                if i == j {
                    continue;
                }
                let pos = moons[j].pos;
                moons[i].apply_gravity(pos);
            }
        }
        moons.iter_mut().for_each(|m| m.apply_velocity());
    }
    let result = moons.iter().map(|m| m.total()).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(mut moons: Vec<Moon>) -> Result<Coord> {
    let start = Instant::now();

    let init_moons = moons.clone();
    let l = moons.len();
    let mut result = 0;
    for step in 1.. {
        for i in 0..l {
            for j in 0..l {
                if i == j {
                    continue;
                }
                let pos = moons[j].pos;
                moons[i].apply_gravity(pos);
            }
        }
        moons.iter_mut().for_each(|m| m.apply_velocity());
        if init_moons.iter().zip(moons.iter()).all(|(m1, m2)| m1 == m2) {
            result = step;
            break;
        }
        if step % 50000000 == 0 {
            println!("{} {:?}", step, start.elapsed());
        }
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: Pos,
    vel: Pos,
}

impl Moon {
    fn apply_gravity(&mut self, other: Pos) {
        self.vel.0 += change_velocity(self.x(), other.0);
        self.vel.1 += change_velocity(self.y(), other.1);
        self.vel.2 += change_velocity(self.z(), other.2);
    }

    fn apply_velocity(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn x(&self) -> Coord {
        self.pos.0
    }

    fn y(&self) -> Coord {
        self.pos.1
    }

    fn z(&self) -> Coord {
        self.pos.2
    }

    fn pot(&self) -> Coord {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }

    fn kin(&self) -> Coord {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }

    fn total(&self) -> Coord {
        self.pot() * self.kin()
    }
}

fn change_velocity(a: Coord, b: Coord) -> Coord {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

impl FromStr for Moon {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let mut r = vec![];
        for part in s.split(", ") {
            let part = part.strip_suffix('>').unwrap_or(part);
            if let Some((_, part)) = part.split_once('=') {
                r.push(part.parse()?)
            }
        }
        Ok(Moon {
            pos: (r[0], r[1], r[2]),
            vel: (0, 0, 0),
        })
    }
}

#[test]
fn example_input() {
    let input = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    let moons = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Moon>>>()
        .unwrap();
    assert_eq!(part1(moons.clone(), 10).unwrap(), 179);
    assert_eq!(part2(moons).unwrap(), 2772);

    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    let moons = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Moon>>>()
        .unwrap();
    assert_eq!(part1(moons.clone(), 100).unwrap(), 1940);
    assert_eq!(part2(moons).unwrap(), 4686774924);
}
