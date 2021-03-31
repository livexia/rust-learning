use std::io::{self, Read, Write};
use std::error::Error;
use std::result;
use std::fmt;
use std::str::FromStr;
use std::collections::BTreeMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let area: Area = input.parse()?;
    part1(&area)?;
    part2(&area)?;

    Ok(())
}

fn part1(area: &Area) -> Result<()> {
    let answer = area.distance.iter().max_by_key(|(_, &d)| d).unwrap().1;
    writeln!(io::stdout(), "part1 answer: {}", answer)?;
    Ok(())
}

fn part2(area: &Area) -> Result<()> {
    let answer = area.distance.iter().fold(0, |sum, (_, &d)| {
        if d > 999 { sum + 1 } else { sum }
    });
    writeln!(io::stdout(), "part2 answer: {}", answer)?;
    Ok(())
}

#[derive(Debug)]
struct Area {
    start: Coordinate,
    north_west: Coordinate,
    south_east: Coordinate,
    grid: BTreeMap<Coordinate, Kind>,
    distance: BTreeMap<Coordinate, i32>,
}

impl Area {
    fn new() -> Self {
        Self { 
            start: Coordinate::new(0, 0),
            north_west: Coordinate::new(0, 0),
            south_east: Coordinate::new(0, 0),
            grid: BTreeMap::new(),
            distance: BTreeMap::new(),
        }
    }

    fn update_area(&mut self) {
        let min_x = self.grid.iter().
            map(|(c, _)| c.x).min().unwrap() - 1;
        let max_x = self.grid.iter().
            map(|(c, _)| c.x).max().unwrap() + 1;
        let min_y = self.grid.iter().
            map(|(c, _)| c.y).min().unwrap() - 1;
        let max_y = self.grid.iter().
            map(|(c, _)| c.y).max().unwrap() + 1;

        self.north_west = Coordinate::new(max_x, max_y);
        self.south_east = Coordinate::new(min_x, min_y);
    }

    fn move_north(&mut self, cur: Coordinate) -> Coordinate {
        let door = cur.north();
        let room = door.north();
        self.grid.insert(door, Kind::HorizontalDoor);
        self.grid.insert(room, Kind::Room);
        self.update_distance(cur, room);
        room
    }

    fn move_south(&mut self, cur: Coordinate) -> Coordinate {
        let door = cur.south();
        let room = door.south();
        self.grid.insert(door, Kind::HorizontalDoor);
        self.grid.insert(room, Kind::Room);
        self.update_distance(cur, room);
        room
    }

    fn move_west(&mut self, cur: Coordinate) -> Coordinate {
        let door = cur.west();
        let room = door.west();
        self.grid.insert(door, Kind::VerticalDoor);
        self.grid.insert(room, Kind::Room);
        self.update_distance(cur, room);
        room
    }

    fn move_east(&mut self, cur: Coordinate) -> Coordinate {
        let door = cur.east();
        let room = door.east();
        self.grid.insert(door, Kind::VerticalDoor);
        self.grid.insert(room, Kind::Room);
        self.update_distance(cur, room);
        room
    }

    fn update_distance(&mut self, cur: Coordinate, next: Coordinate) {
        let cur_distance = if let Some(&cur_distance) = self.distance.get(&cur) {
            cur_distance
        } else {
            self.distance.insert(cur, 0);
            0
        };
        if let Some(&next_distance) = self.distance.get(&next) {
            self.distance.insert(next, next_distance.min(cur_distance + 1));
        } else {
            self.distance.insert(next, cur_distance + 1);
        };
    }
}

impl FromStr for Area {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if !s.starts_with("^") || !s.ends_with("$") {
            return err!("input does not start with '^' and end with '$'");
        }
        let chars: Vec<char> = s.chars()
            .filter(|&c| c != ' ')
            .collect();
        let mut area = Area::new();
        let start = Coordinate::new(0, 0);

        let mut stack = vec![];
        stack.push(start);

        let mut cur_index = 0;
        let mut new_start = vec![];
        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            let cur_char = chars[cur_index];
            match cur_char {
                '^' => {
                    area.grid.insert(start, Kind::Room);
                    stack.push(cur)
                },
                'N' => stack.push(area.move_north(cur)),
                'S' => stack.push(area.move_south(cur)),
                'W' => stack.push(area.move_west(cur)),
                'E' => stack.push(area.move_east(cur)),
                '(' => {
                    new_start.push(cur);
                    stack.push(cur);
                }
                ')' => stack.push(new_start.pop().unwrap()),
                '|' => stack.push(*new_start.last().unwrap()),
                '$' => continue,
                _ => return err!("unrecognized regex: '{}'", cur_char)
            };
            cur_index += 1;
        }
        
        area.update_area();
        Ok(area)
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (self.south_east.y..=self.north_west.y).rev() {
            for x in (self.south_east.x..=self.north_west.x).rev() {
                if Coordinate::new(x, y) == self.start {
                    write!(f, "X")?;
                } else {
                    match self.grid.get(&Coordinate::new(x, y)) {
                        Some(k) => write!(f, "{}", k)?,
                        None => write!(f, "#")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn north(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }

    fn south(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }
    
    fn east(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }
    
    fn west(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    // Wall,
    Room,
    VerticalDoor,
    HorizontalDoor,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Kind::Wall => write!(f, "#")?,
            Kind::Room => write!(f, ".")?,
            Kind::VerticalDoor => write!(f, "|")?,
            Kind::HorizontalDoor => write!(f, "-")?,
        }
        Ok(())
    }
}