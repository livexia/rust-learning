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

fn part1(mut moons: Vec<Moon>, steps: usize) -> Result<u64> {
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
    fn get_pos(moons: &[Moon], axis: u8) -> Vec<Coord> {
        moons
            .iter()
            .map(|m| match axis {
                0 => m.x(),
                1 => m.y(),
                2 => m.z(),
                _ => unreachable!(),
            })
            .collect()
    }

    fn get_vel(moons: &[Moon], axis: u8) -> Vec<Coord> {
        moons
            .iter()
            .map(|m| match axis {
                0 => m.vel.0,
                1 => m.vel.1,
                2 => m.vel.2,
                _ => unreachable!(),
            })
            .collect()
    }
    let init_pos = get_pos(moons, axis);
    let init_vel = get_vel(moons, axis);
    let l = moons.len();
    for step in 1.. {
        for i in 0..l {
            for j in i + 1..l {
                apply_gravity_on_one_axis(moons, i, j, axis);
            }
        }
        moons
            .iter_mut()
            .for_each(|m| m.apply_velocity_on_one_axis(axis));
        if get_pos(moons, axis) == init_pos && get_vel(moons, axis) == init_vel {
            return step;
        }
    }
    0
}

fn lcm(a: u64, b: u64) -> u64 {
    let (a, b) = if a > b { (b, a) } else { (a, b) };
    let mut r = b;
    while r % a != 0 {
        r += b;
    }
    r
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

    fn apply_velocity_on_one_axis(&mut self, axis: u8) {
        match axis {
            0 => self.pos.0 += self.vel.0,
            1 => self.pos.1 += self.vel.1,
            2 => self.pos.2 += self.vel.2,
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

    fn pot(&self) -> u64 {
        self.pos.0.abs() as u64 + self.pos.1.abs() as u64 + self.pos.2.abs() as u64
    }

    fn kin(&self) -> u64 {
        self.vel.0.abs() as u64 + self.vel.1.abs() as u64 + self.vel.2.abs() as u64
    }

    fn total(&self) -> u64 {
        self.pot() * self.kin()
    }
}

fn apply_gravity_on_one_axis(moons: &mut [Moon], a: usize, b: usize, axis: u8) {
    match axis {
        0 => {
            let dx = change_velocity(moons[a].x(), moons[b].x());
            moons[a].vel.0 += dx;
            moons[b].vel.0 -= dx;
        }
        1 => {
            let dy = change_velocity(moons[a].y(), moons[b].y());
            moons[a].vel.1 += dy;
            moons[b].vel.1 -= dy;
        }
        2 => {
            let dz = change_velocity(moons[a].z(), moons[b].z());
            moons[a].vel.2 += dz;
            moons[b].vel.2 -= dz;
        }
        _ => unreachable!(),
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
