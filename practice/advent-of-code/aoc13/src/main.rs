  
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
    loop {
    // for _ in 0..51 {
        // writeln!(io::stdout(), "tick: {}\n{}\n", map.tick, map.string())?;
        if let Some(location) = map.location_of_crash() {
            writeln!(io::stdout(), "the location of the first crash: {},{}", location.1, location.0)?;
            // writeln!(io::stdout(), "tick: {}\n{}\n", map.tick, map.string())?;

            break;
        } else {
            map.next()?;
        }
        
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Track {
    Vertical,
    Horizontal,
    Right,
    Left,
    Intersection,
    None,
}

impl Track {
    fn new(c: char) -> Self {
        match c {
            '-' | '>' | '<' => Track::Horizontal,
            '|' | 'v' | '^'  => Track::Vertical,
            '/' => Track::Right,
            '\\' => Track::Left,
            '+' => Track::Intersection,
            _ => Track::None,
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
        let location = (i, j);
        let direction;
        let turn_times = 0;
        match c {
            '>' => direction = Direction::Right,
            '<' => direction = Direction::Left,
            '^' => direction = Direction::Up,
            'v' => direction = Direction::Down,
            _ => return None,
        }
        Some(Cart { direction, location, turn_times })
    }

    fn turn(&mut self, direction: Option<Direction>) {
        match self.direction {
            Direction::Left => {
                match direction {
                    Some(Direction::Left) => { self.direction = Direction::Down; self.location.0 += 1; }
                    Some(Direction::Right) => { self.direction = Direction::Up; self.location.0 -= 1; }
                    None => self.location.1 -= 1,
                    _ => ()
                }
            }
            Direction::Right => {
                match direction {
                    Some(Direction::Left) => { self.direction = Direction::Up; self.location.0 -= 1; }
                    Some(Direction::Right) => { self.direction = Direction::Down; self.location.0 += 1; }
                    None => self.location.1 += 1,
                    _ => ()
                }
            }
            Direction::Down => {
                match direction {
                    Some(Direction::Left) => { self.direction = Direction::Right; self.location.1 += 1; }
                    Some(Direction::Right) => { self.direction = Direction::Left; self.location.1 -= 1; }
                    None => self.location.0 += 1,
                    _ => ()
                }
            }
            Direction::Up => {
                match direction {
                    Some(Direction::Left) => { self.direction = Direction::Left; self.location.1 -= 1; }
                    Some(Direction::Right) => { self.direction = Direction::Right; self.location.1 += 1; }
                    None => self.location.0 -= 1,
                    _ => ()
                }
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    tracks: Vec<Vec<Track>>,
    carts: Vec<Cart>,
    tick: u32
}

impl Map {
    fn new() -> Self {
        Self {
            tracks: Vec::new(),
            carts: Vec::new(),
            tick: 0,
        }
    }

    fn next(&mut self) -> Result<()> {
        let vertical = vec![Some(&Track::Vertical), Some(&Track::Intersection)];
        let horizontal = vec![Some(&Track::Horizontal), Some(&Track::Intersection)];
        self.carts.sort_by(
            |c1, c2| 
            if c1.location.0 != c1.location.0 { c1.location.0.cmp(&c2.location.0) } 
            else { c1.location.1.cmp(&c2.location.1)});
        for cart in &mut self.carts {
            let cur = cart.location;
            let track = &self.tracks[cur.0][cur.1];
            match &track {
                Track::Horizontal => { 
                    match cart.direction {
                        Direction::Right =>  cart.location.1 += 1,
                        Direction::Left => cart.location.1 -= 1,
                        _ => return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track)
                    }
                }
                Track::Vertical => {
                    match cart.direction {
                        Direction::Up => cart.location.0 -= 1,
                        Direction::Down => cart.location.0 += 1,
                        _ => return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track)

                    }
                }
                Track::Right => {
                    match cart.direction {
                        Direction::Right => if let Some(row) = self.tracks.get(cur.0 - 1) {
                            if vertical.contains(&row.get(cur.1)) {
                                cart.turn(Some(Direction::Left));
                            } else { 
                                return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                            }
                        },
                        Direction::Down => if horizontal.contains(&self.tracks[cur.0].get(cur.1 - 1)) {
                            cart.turn(Some(Direction::Right));
                        } else { 
                            return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                        },
                        Direction::Left => if let Some(row) = self.tracks.get(cur.0 + 1) {
                            if vertical.contains(&row.get(cur.1)) {
                                cart.turn(Some(Direction::Left));
                            } else { 
                                return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                            }
                        },
                        Direction::Up => if horizontal.contains(&self.tracks[cur.0].get(cur.1 + 1)) {
                            cart.turn(Some(Direction::Right));
                        } else { 
                            return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                        }, 
                    }
                }
                Track::Left => {
                    match cart.direction {
                        Direction::Right => if let Some(row) = self.tracks.get(cur.0 + 1) {
                            if vertical.contains(&row.get(cur.1)) {
                                cart.turn(Some(Direction::Right));
                            } else { 
                                return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                            }
                        },
                        Direction::Down => if horizontal.contains(&self.tracks[cur.0].get(cur.1 + 1)) {
                            cart.turn(Some(Direction::Left));
                        } else { 
                            return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                        },
                        Direction::Left => if let Some(row) = self.tracks.get(cur.0 - 1) {
                            if vertical.contains(&row.get(cur.1)) {
                                cart.turn(Some(Direction::Right));
                            } else { 
                                return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                            }
                        },
                        Direction::Up => if horizontal.contains(&self.tracks[cur.0].get(cur.1 - 1)) {
                            cart.turn(Some(Direction::Left));
                        } else { 
                            return err!("Something goes wrong: {:?} {:?} {:?}", cart, cur, track) 
                        },
                    }
                }
                Track::Intersection => {
                    match cart.turn_times % 3 {
                        0 => cart.turn(Some(Direction::Left)),
                        1 => cart.turn(None),
                        2 => cart.turn(Some(Direction::Right)),
                        _ => unreachable!(),
                    }
                    cart.turn_times += 1;
                }
                Track::None => { return err!("Something goes wrong, cart on the None track: {:?} {:?} {:?}", cart, cur, track) }
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
        let mut map: Vec<Vec<char>> = self.tracks.iter().map(|row| row.iter().map(
            |f| match f {
                Track::Horizontal => '-', 
                Track::Vertical => '|',
                Track::Right => '/',
                Track::Left => '\\',
                Track::Intersection => '+',
                Track::None => ' '
            }
        ).collect()).collect();
        for cart in &self.carts {
            let p = cart.location;
            if ['v', '^', '<', '>'].contains(&map[p.0][p.1]) {
                map[p.0][p.1] = 'X'
            } else {
                map[p.0][p.1] = match cart.direction {
                    Direction::Left => '<',
                    Direction::Right => '>',
                    Direction::Up => '^',
                    Direction::Down => 'v'
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
            map.tracks.push(vec![]);
            for (j, c) in line.chars().enumerate() {
                map.tracks[i].push(Track::new(c));
                if let Some(c) = Cart::new(c, i, j) {
                    map.carts.push(c);
                }
            }
        }
        Ok(map)
    }
}