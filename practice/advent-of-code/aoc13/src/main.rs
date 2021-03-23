use std::{io::{self, Read, Write}, vec};
use std::error::Error;
use std::result;
use std::str::FromStr;
use std::time::Instant;
use std::collections::HashSet;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut map: Map = input.parse()?;
    // loop {
    for _ in 0..120 {
        if let Some(location) = map.location_of_crash() {
            writeln!(io::stdout(), "the location of the first crash: {},{}", location.1, location.0)?;
            // writeln!(io::stdout(), "tick: {}\n{}\n", map.tick, map.string())?;

            break;
        } else {
            map.next()?;
        }
        writeln!(io::stdout(), "tick: {}\n{}\n", map.tick, map.string())?;
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Track {
    Vertical,
    Horizontal,
    Forward,
    Backward,
    Intersection,
    Empty,
}

impl Track {
    fn new(c: char) -> Self {
        match c {
            '-' | '>' | '<' => Track::Horizontal,
            '|' | 'v' | '^'  => Track::Vertical,
            '/' => Track::Forward,
            '\\' => Track::Backward,
            '+' => Track::Intersection,
            _ => Track::Empty,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

type Location = (usize, usize);

#[derive(Debug)]
struct Cart {
    direction: Direction,
    location: Location,
    turn_times: u32
}

impl Cart {

    fn new(c: char, i: usize, j: usize) -> Option<Self> {
        use crate::Direction::*;
        
        let location = (i, j);
        let direction;
        let turn_times = 0;
        match c {
            '>' => direction = Right,
            '<' => direction = Left,
            '^' => direction = Up,
            'v' => direction = Down,
            _ => return None,
        }
        Some(Cart { direction, location, turn_times })
    }

    fn turn(&mut self, direction: Direction) {
        use crate::Direction::*;

        self.direction = match (&self.direction, direction) {
            (Left, Left) => Down,
            (Left, Right) => Up,
            (Right, Left) => Up,
            (Right, Right) => Down,
            (Up, Left) => Left,
            (Up, Right) => Right,
            (Down, Left) => Right,
            (Down, Right) => Left,
            _ => return,
        }
    }

    fn intersection(&mut self) {
        use crate::Direction::*;
        let which = self.turn_times % 3;
        self.turn_times += 1;
        match which {
            0 => self.turn(Left),
            1 => (),
            2 => self.turn(Right),
            _ => unreachable!()
        }
    }
}


#[derive(Debug)]
struct Grid {
    tracks: Vec<Vec<Track>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            tracks: Vec::new(),
        }
    }
    
    fn up(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.location;
        if coord.0 == 0 {
            err!("cannot go up")
        } else {
            match &self.tracks[coord.0 - 1][coord.1] {
                Track::Empty => return err!("cannot go up"),
                d => { cart.location.0 -= 1; return Ok(d) },
            }
        }
    }

    fn down(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.location;
        if coord.0 == self.tracks.len() - 1 {
            err!("cannot go down")
        } else {
            match &self.tracks[coord.0 + 1][coord.1] {
                Track::Empty => return err!("cannot go down"),
                d => { cart.location.0 += 1; return Ok(d) },
            }
        }
    }

    fn left(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.location;
        if coord.1 == 0 {
            err!("cannot go left")
        } else {
            match &self.tracks[coord.0][coord.1 - 1] {
                Track::Empty => return err!("cannot go left"),
                d => { cart.location.1 -= 1; return Ok(d) },
            }
        }
    }

    fn right(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.location;
        if coord.1 == self.tracks[coord.0].len() - 1 {
            err!("cannot go right")
        } else {
            match &self.tracks[coord.0][coord.1 + 1] {
                Track::Empty => return err!("cannot go right"),
                d => { cart.location.1 += 1; return Ok(d) },
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Grid,
    carts: Vec<Cart>,
    tick: u32
}

impl Map {
    fn new() -> Self {
        Self {
            grid: Grid::new(),
            carts: Vec::new(),
            tick: 0,
        }
    }

    fn next(&mut self) -> Result<()> {
        use crate::Direction::*;
        use crate::Track::*;

        self.carts.sort_by(
            |c1, c2| 
            if c1.location.0 != c1.location.0 { c1.location.0.cmp(&c2.location.0) } 
            else { c1.location.1.cmp(&c2.location.1)});
        for cart in &mut self.carts {
            let cur = cart.location;
            let track = &self.grid.tracks[cur.0][cur.1];

            let next_track = match (&cart.direction, track) {
                (_, Empty) => return err!("invalid transition on empty"),
                (Left, Vertical) => return err!("cannot go up on vertical"),
                (Left, _) => self.grid.left(cart)?,
                (Right, Vertical) => return err!("cannot go up on vertical"),
                (Right, _) => self.grid.right(cart)?,
                (Up, Horizontal) => return err!("cannot go up on horizontal"),
                (Up, _) => self.grid.up(cart)?,
                (Down, Horizontal) => return err!("cannot go down on horizontal"),
                (Down, _) => self.grid.down(cart)?,
            };

            match (&cart.direction, next_track) {
                (_, Empty) => return err!("invalid transition on empty"),
                (_, Intersection) => cart.intersection(),
                (Left, Vertical) => (),
                (Left, Horizontal) => (),
                (Left, Forward) => cart.direction = Down,
                (Left, Backward) => cart.direction = Up,
                (Right, Vertical) => (),
                (Right, Horizontal) => (),
                (Right, Forward) => cart.direction = Up,
                (Right, Backward) => cart.direction = Down,
                (Up, Vertical) => (),
                (Up, Horizontal) => (),
                (Up, Forward) => cart.direction = Right,
                (Up, Backward) => cart.direction = Left,
                (Down, Vertical) => (),
                (Down, Horizontal) => (),
                (Down, Forward) => cart.direction = Left,
                (Down, Backward) => cart.direction = Right, 
            }
        }
        self.tick += 1;
        Ok(())
    }

    fn location_of_crash(&mut self) -> Option<Location> {
        let mut cart_location: HashSet<Location> = HashSet::new();
        self.carts.sort_by(
            |c1, c2| 
            if c1.location.0 != c1.location.0 { c1.location.0.cmp(&c2.location.0) } 
            else { c1.location.1.cmp(&c2.location.1)});
        for cart in &self.carts {
            if cart_location.contains(&cart.location) {
                return Some(cart.location)
            }
            cart_location.insert(cart.location);
        }
        None
    }

    fn string(&self) -> String {
        use crate::Direction::*;
        use crate::Track::*;

        let mut map: Vec<Vec<char>> = self.grid.tracks.iter().map(|row| row.iter().map(
            |f| match f {
                Horizontal => '-', 
                Vertical => '|',
                Forward => '/',
                Backward => '\\',
                Intersection => '+',
                Empty => ' '
            }
        ).collect()).collect();
        for cart in &self.carts {
            let p = cart.location;
            if ['v', '^', '<', '>'].contains(&map[p.0][p.1]) {
                map[p.0][p.1] = 'X'
            } else {
                map[p.0][p.1] = match cart.direction {
                    Left => '<',
                    Right => '>',
                    Up => '^',
                    Down => 'v'
                }
            }
        }
        map.iter().map(|row| String::from_iter(row)).collect::<Vec<String>>().join("\n")
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = Map::new();
        for (i, line) in s.lines().enumerate() {
            map.grid.tracks.push(vec![]);
            for (j, c) in line.chars().enumerate() {
                map.grid.tracks[i].push(Track::new(c));
                if let Some(c) = Cart::new(c, i, j) {
                    map.carts.push(c);
                }
            }
        }
        Ok(map)
    }
}