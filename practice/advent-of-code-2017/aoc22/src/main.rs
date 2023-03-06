use std::collections::HashSet;
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
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (grid, carrier) = parse_inpit(&input);

    part1(&grid, &carrier)?;
    // part2()?;
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

#[test]
fn example_input() {
    let input = "..#
        #..
        ...";
    let (grid, carrier) = parse_inpit(input);
    assert_eq!(part1(&grid, &carrier).unwrap(), 5587);
}
