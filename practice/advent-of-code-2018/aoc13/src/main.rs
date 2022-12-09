use std::io::{self, Read, Write};
use std::error::Error;
use std::mem;
use std::result;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut map: Map = input.parse()?;
    loop {
        let crashes = map.next()?;
        if !crashes.is_empty() {
            let c = crashes[0];
            writeln!(io::stdout(), "the location of the first crash: {},{}", c.1, c.0)?;
            break;
        }
    }


    loop {
        map.next()?;
        if map.carts.len() == 1 {
            let (&last_cart, _) = map.carts.clone().iter().next().unwrap();
            writeln!(io::stdout(), "the location of the last cart: {},{}", last_cart.1, last_cart.0)?;
            break;
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Crashed,
}

type Coordinate = (usize, usize);

#[derive(Debug, Clone)]
struct Cart {
    direction: Direction,
    coord: Coordinate,
    turn_times: u32
}

impl Cart {

    fn new(c: char, i: usize, j: usize) -> Option<Self> {
        use crate::Direction::*;
        
        let coord = (i, j);
        let direction;
        let turn_times = 0;
        match c {
            '>' => direction = Right,
            '<' => direction = Left,
            '^' => direction = Up,
            'v' => direction = Down,
            _ => return None,
        }
        Some(Cart { direction, coord, turn_times })
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

    fn is_crashed(&self) -> bool {
        self.direction == Direction::Crashed
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
        let coord = cart.coord;
        if coord.0 == 0 {
            err!("cannot go up")
        } else {
            match &self.tracks[coord.0 - 1][coord.1] {
                Track::Empty => return err!("cannot go up"),
                d => { cart.coord.0 -= 1; return Ok(d) },
            }
        }
    }

    fn down(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.coord;
        if coord.0 == self.tracks.len() - 1 {
            err!("cannot go down")
        } else {
            match &self.tracks[coord.0 + 1][coord.1] {
                Track::Empty => return err!("cannot go down"),
                d => { cart.coord.0 += 1; return Ok(d) },
            }
        }
    }

    fn left(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.coord;
        if coord.1 == 0 {
            err!("cannot go left")
        } else {
            match &self.tracks[coord.0][coord.1 - 1] {
                Track::Empty => return err!("cannot go left"),
                d => { cart.coord.1 -= 1; return Ok(d) },
            }
        }
    }

    fn right(&self, cart: &mut Cart) -> Result<&Track> {
        let coord = cart.coord;
        if coord.1 == self.tracks[coord.0].len() - 1 {
            err!("cannot go right")
        } else {
            match &self.tracks[coord.0][coord.1 + 1] {
                Track::Empty => return err!("cannot go right"),
                d => { cart.coord.1 += 1; return Ok(d) },
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Grid,
    carts: BTreeMap<Coordinate, Cart>,
    tick: u32
}

impl Map {
    fn new() -> Self {
        Self {
            grid: Grid::new(),
            carts: BTreeMap::new(),
            tick: 0,
        }
    }

    fn next(&mut self) -> Result<Vec<Coordinate>> {
        use crate::Direction::*;
        use crate::Track::*;

        let mut crashes = HashSet::new();
        let mut previous_carts = mem::replace(&mut self.carts, BTreeMap::new());
        
        for (cur, mut cart) in previous_carts.clone() {
            if crashes.contains(&cur) {
                continue;
            }
            let track = &self.grid.tracks[cur.0][cur.1];
            let next_track = match (&cart.direction, track) {
                (_, Empty) => return err!("invalid transition on empty"),
                (Crashed, _) => track,
                (Left, Vertical) => return err!("cannot go up on vertical"),
                (Left, _) => self.grid.left(&mut cart)?,
                (Right, Vertical) => return err!("cannot go up on vertical"),
                (Right, _) => self.grid.right(&mut cart)?,
                (Up, Horizontal) => return err!("cannot go up on horizontal"),
                (Up, _) => self.grid.up(&mut cart)?,
                (Down, Horizontal) => return err!("cannot go down on horizontal"),
                (Down, _) => self.grid.down(&mut cart)?,
            };

            match (&cart.direction, next_track) {
                (_, Empty) => return err!("invalid transition on empty"),
                (Crashed, _) => (),
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
            let next = cart.coord;
            assert!(!cart.is_crashed());
            if previous_carts.contains_key(&next) || self.carts.contains_key(&next) {
                self.carts.remove(&next);
                crashes.insert(next);
            } else {
                assert!(!self.carts.contains_key(&next));
                self.carts.insert(next, cart);
            }
            previous_carts.remove(&cur);
        }
        self.tick += 1;
        Ok(crashes.into_iter().collect())
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
        for (_, cart) in &self.carts {
            let p = cart.coord;
            if ['v', '^', '<', '>'].contains(&map[p.0][p.1]) {
                map[p.0][p.1] = 'X'
            } else {
                map[p.0][p.1] = match cart.direction {
                    Left => '<',
                    Right => '>',
                    Up => '^',
                    Down => 'v',
                    Crashed => 'X',
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
                    map.carts.insert((i, j), c);
                }
            }
        }
        Ok(map)
    }
}