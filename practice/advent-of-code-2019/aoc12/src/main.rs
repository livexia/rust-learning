use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = i16;
type Pos = (Coord, Coord, Coord);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let moons = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Moon>>>()?;

    part1(moons.clone(), 1000)?;
    part2(moons)?;
    Ok(())
}

fn part1(mut moons: Vec<Moon>, steps: usize) -> Result<u16> {
    let start = Instant::now();

    let l = moons.len();
    for _ in 0..steps {
        for i in 0..l {
            for j in i + 1..l {
                apply_gravity(&mut moons, i, j);
            }
        }
        moons.iter_mut().for_each(|m| m.apply_velocity());
    }
    let result = moons.iter().map(|m| m.total()).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(mut moons: Vec<Moon>) -> Result<u64> {
    let start = Instant::now();

    let step_x = find_step_for_one_axis(&mut moons, 0);
    let step_y = find_step_for_one_axis(&mut moons, 1);
    let step_z = find_step_for_one_axis(&mut moons, 2);

    let result = lcm(lcm(step_x, step_y), step_z);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

// https://www.reddit.com/r/adventofcode/comments/e9jxh2/comment/faje38l
fn find_step_for_one_axis(moons: &mut [Moon], axis: u8) -> u64 {
    let init_pos: Vec<_> = moons.iter().map(|m| m.get_pos(axis)).collect();
    let init_vel = vec![0; moons.len()];

    let l = moons.len();
    let mut pos = init_pos.clone();
    let mut vel = init_vel.clone();
    for step in 1.. {
        for i in 0..l {
            for j in i + 1..l {
                apply_gravity_on_one_axis(&mut vel, &pos, i, j);
            }
        }
        apply_velocity_on_one_axis(&mut pos, &vel);
        if vel == init_vel && pos == init_pos {
            return step;
        }
    }
    0
}

// From https://doc.rust-lang.org/std/ops/trait.Div.html
// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: Pos,
    vel: Pos,
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn get_pos(&self, axis: u8) -> Coord {
        match axis {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => unreachable!(),
        }
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

    fn pot(&self) -> u16 {
        self.pos.0.unsigned_abs() + self.pos.1.unsigned_abs() + self.pos.2.unsigned_abs()
    }

    fn kin(&self) -> u16 {
        self.vel.0.unsigned_abs() + self.vel.1.unsigned_abs() + self.vel.2.unsigned_abs()
    }

    fn total(&self) -> u16 {
        self.pot() * self.kin()
    }
}

fn apply_gravity_on_one_axis(vel: &mut [Coord], pos: &[Coord], i: usize, j: usize) {
    let d = change_velocity(pos[i], pos[j]);
    vel[i] += d;
    vel[j] -= d;
}

fn apply_velocity_on_one_axis(pos: &mut [Coord], vel: &[Coord]) {
    for (p, v) in pos.iter_mut().zip(vel.iter()) {
        *p += v
    }
}

fn apply_gravity(moons: &mut [Moon], a: usize, b: usize) {
    let dx = change_velocity(moons[a].x(), moons[b].x());
    let dy = change_velocity(moons[a].y(), moons[b].y());
    let dz = change_velocity(moons[a].z(), moons[b].z());
    moons[a].vel.0 += dx;
    moons[a].vel.1 += dy;
    moons[a].vel.2 += dz;
    moons[b].vel.0 -= dx;
    moons[b].vel.1 -= dy;
    moons[b].vel.2 -= dz;
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
