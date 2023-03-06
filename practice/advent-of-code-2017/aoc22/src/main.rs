use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);
type Grid = HashSet<Coord>;
type FlagGrid = HashMap<Coord, Flag>;

fn parse_inpit(input: &str) -> (Grid, Carrier) {
    let mut grid = Grid::new();

    let (mut x, mut y) = (0, 0);
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.trim().char_indices() {
            if c == '#' {
                grid.insert((i as i32, j as i32));
            }
            y = y.max(j as i32);
        }
        x = x.max(i as i32);
    }

    (grid, Carrier::new((x / 2, y / 2)))
}

#[derive(Clone)]
struct Carrier {
    pos: Coord,
    facing: u8, // 0 up, 1 right, 2 down, 3 left
}

impl Carrier {
    fn new(c: Coord) -> Self {
        Self { pos: c, facing: 0 }
    }

    fn turn_left(&mut self) {
        // counter clockwise
        self.facing = (self.facing + 4 - 1) % 4;
    }

    fn turn_right(&mut self) {
        // clockwise
        self.facing = (self.facing + 1) % 4;
    }

    fn move_forward(&mut self) {
        match self.facing {
            0 => self.pos.0 -= 1,
            1 => self.pos.1 += 1,
            2 => self.pos.0 += 1,
            3 => self.pos.1 -= 1,
            _ => unreachable!("Wrong facing"),
        }
    }

    fn burst(&mut self, grid: &mut Grid) -> bool {
        let infected = grid.contains(&self.pos);
        if infected {
            self.turn_right();
            grid.remove(&self.pos);
        } else {
            self.turn_left();
            grid.insert(self.pos);
        }
        self.move_forward();
        !infected
    }

    fn reverse(&mut self) {
        self.facing = (self.facing + 2) % 4;
    }

    fn burst_with_evolves(&mut self, grid: &mut FlagGrid) -> bool {
        use Flag::*;
        let &flag = grid.get(&self.pos).unwrap_or(&Flag::Clean);
        let mut infected = false;
        match flag {
            Clean => self.turn_left(),
            Infected => self.turn_right(),
            Flagged => self.reverse(),
            Weakened => infected = true,
        }
        grid.insert(self.pos, flag.next());
        self.move_forward();
        infected
    }
}

#[derive(Clone, Copy, Debug)]
enum Flag {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Flag {
    fn next(self) -> Self {
        use Flag::*;

        match self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (grid, carrier) = parse_inpit(&input);

    part1(&grid, &carrier)?;
    part2(&grid, &carrier)?;
    Ok(())
}

fn part1(grid: &Grid, carrier: &Carrier) -> Result<usize> {
    let start = Instant::now();

    let mut grid = grid.to_owned();
    let mut carrier = carrier.to_owned();
    let result = (0..10_000).filter(|_| carrier.burst(&mut grid)).count();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(grid: &Grid, carrier: &Carrier) -> Result<usize> {
    let start = Instant::now();

    let mut grid = grid.iter().map(|c| (*c, Flag::Infected)).collect();
    let mut carrier = carrier.to_owned();
    let result = (0..10_000_000)
        .filter(|_| carrier.burst_with_evolves(&mut grid))
        .count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[test]
fn example_input() {
    let input = "..#
        #..
        ...";
    let (grid, carrier) = parse_inpit(input);
    assert_eq!(part1(&grid, &carrier).unwrap(), 5587);
    assert_eq!(part2(&grid, &carrier).unwrap(), 2511944);
}
