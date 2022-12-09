#[macro_use]
extern crate lazy_static;

use std::{io::{self, Read, Write}, u32};
use std::error::Error;
use std::result;
use std::str::{self, FromStr};

use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let points = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>>>()?;
    if points.is_empty() {
        return err!("no point given");
    }

    let mut grid = Grid::new(points)?;

    let mut answer = grid.string();
    let mut second = 0;
    for _ in 0..50_000 {
        grid.step();
        let (w, h) = grid.dimensions();
        if w <= 80 && h <= 80 {
            let temp = grid.string();
            if temp.len() < answer.len() {
                second = grid.seconds;
                answer = temp;
            }
        }
    }
    writeln!(io::stdout(), "Result")?;
    writeln!(io::stdout(), "seconds: {}", second)?;
    writeln!(io::stdout(), "{}", answer)?;
    
    Ok(())
}

#[derive(Debug)]
struct Grid {
    points: Vec<Point>,
    seconds: u32,
    max_height: i32,
    max_width: i32,
    min_height: i32,
    min_width: i32
}

impl Grid {
    fn new(points: Vec<Point>) -> Result<Self> {
        if points.is_empty() {
            return err!("no points given");
        }
        let mut grid = Self { points, seconds: 0, max_height: 0, max_width: 0, min_height: 0, min_width: 0 };
        grid.update_bounds();
        Ok(grid)
    }

    fn update_bounds(&mut self) {
        let fp = &self.points[0];
        self.max_height = self.points.iter().fold(fp.y, |y, p| y.max(p.y));
        self.min_height = self.points.iter().fold(fp.y, |y, p| y.min(p.y));
        self.max_width = self.points.iter().fold(fp.x, |x, p| x.max(p.x));
        self.min_width = self.points.iter().fold(fp.x, |x, p| x.min(p.x));
    }

    fn normal_x(&self, x: i32) -> u32 {
        (x - self.min_width) as u32
    }

    fn normal_y(&self, y: i32) -> u32 {
        (y - self.min_height) as u32
    }

    fn width(&self) -> usize {
        (self.max_width - self.min_width + 1) as usize
    }

    fn height(&self) -> usize {
        (self.max_height - self.min_height + 1) as usize
    }

    fn dimensions(&mut self) -> (usize, usize) {
        self.update_bounds();
        (self.width(), self.height())
    }

    fn string(&mut self) -> String {
        self.update_bounds();
        let mut grid = vec![vec![b'.'; self.width()]; self.height()];
        for p in &self.points {
            let x = self.normal_x(p.x);
            let y = self.normal_y(p.y);
            grid[y as usize][x as usize] = b'#';
        }
        let mut buf = String::new();
        for row in grid {
            buf.push_str(str::from_utf8(&row).unwrap());
            buf.push('\n');
        }

        buf
    }

    fn step(&mut self) {
        self.seconds += 1;
        for p in &mut self.points {
            p.x += p.vx;
            p.y += p.vy;
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                position=<\s*
                (?P<x>[-0-9]+)
                ,\s+
                (?P<y>[-0-9]+)
                >\s+velocity=<\s*
                (?P<v_x>[-0-9]+)
                ,\s+
                (?P<v_y>[-0-9]+)>
            ").unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecongnized point"),
            Some(caps) => caps,
        };
        Ok(Point {
            x: caps["x"].parse()?, 
            y: caps["y"].parse()?,
            vx: caps["v_x"].parse()?,
            vy: caps["v_y"].parse()?,
        })
    }
}