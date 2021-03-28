use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt;
use std::result;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut area: Area = input.parse()?;

    part1(area.clone())?;

    // println!("{}", area);
    for _ in 0..1000 {
        area.step();
        writeln!(
            io::stdout(), 
            "minute : {:4}, resource value: {}", 
            area.minute, area.resource_value())?;
    }

    Ok(())
}

fn part1(mut area: Area) -> Result<()> {
    for _ in 0..10 {
        area.step();
    }
    writeln!(io::stdout(), "part1 answer: {}", area.resource_value())?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Area {
    minute: u32,
    height: usize,
    width: usize,
    grid: Vec<Vec<Arce>>
}

impl Area {

    fn resource_value(&self) -> i32 {
        let mut wooded = 0;
        let mut lumberyards = 0;
        for row in &self.grid {
            for a in row {
                match a {
                    Arce::Open => (),
                    Arce::Tree => wooded += 1,
                    Arce::Lumberyard => lumberyards += 1,
                }
            }
        }
        wooded * lumberyards
    }

    fn step(&mut self) {
        let mut grid = self.grid.clone();

        for i in 0..self.height {
            for j in 0..self.width {
                match self.grid[i][j] {
                    Arce::Open => {
                        let tree = self.near(i, j)
                            .iter()
                            .fold(0, |sum, &a| {
                                if a == Arce::Tree { sum + 1 } else { sum }
                            });
                        if tree > 2 {
                            grid[i][j] = Arce::Tree
                        }
                    }
                    Arce::Tree => {
                        let lumberyard = self.near(i, j)
                            .iter()
                            .fold(0, |sum, &a| {
                                if a == Arce::Lumberyard { sum + 1 } else { sum }
                            });
                        if lumberyard > 2 {
                            grid[i][j] = Arce::Lumberyard
                        }
                    }
                    Arce::Lumberyard => {
                        let lumberyard = self.near(i, j)
                            .iter()
                            .fold(0, |sum, &a| {
                                if a == Arce::Lumberyard { sum + 1 } else { sum }
                            });
                        let tree = self.near(i, j)
                            .iter()
                            .fold(0, |sum, &a| {
                                if a == Arce::Tree { sum + 1 } else { sum }
                            });
                        if tree  > 0 && lumberyard > 0 {
                            grid[i][j] = Arce::Lumberyard
                        } else {
                            grid[i][j] = Arce::Open
                        }
                    }
                }
            }
        }
        
        self.minute += 1;
        self.grid = grid;
    }
    
    fn near(&self, x: usize, y: usize) -> Vec<Arce> {
        let mut near = vec![];

        if x > 0 {
            near.push(self.grid[x-1][y])
        }
        if y > 0 {
            near.push(self.grid[x][y-1])
        }
        if x + 1 < self.height {
            near.push(self.grid[x+1][y])
        }
        if y + 1 < self.width {
            near.push(self.grid[x][y+1])
        }
        if x > 0 && y > 0 {
            near.push(self.grid[x-1][y-1])
        }
        if x > 0 && y + 1 < self.width {
            near.push(self.grid[x-1][y+1])
        }
        if x + 1 < self.height && y > 0 {
            near.push(self.grid[x+1][y-1])
        }
        if x + 1 < self.height && y + 1 < self.width {
            near.push(self.grid[x+1][y+1])
        }
        near
    }
}

impl FromStr for Area {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().map(|l| l.trim()).collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut grid = vec![vec![Arce::Open; width]; height];
        for i in 0..height {
            for j in 0..width {
                grid[i][j] = lines[i][j..j+1].parse()?;
            }
        }
        Ok(Self { minute: 0, height, width, grid })
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.minute == 0 {
            writeln!(f, "Initial state:")?;
        } else {
            writeln!(f, "After {} minute:",self.minute)?;
        }
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", self.grid[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arce {
    Open,
    Tree,
    Lumberyard,
}

impl FromStr for Arce {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::Open),
            "|" => Ok(Self::Tree),
            "#" => Ok(Self::Lumberyard),
            _ => err!("wrong arce: {}", s)
        }
    }
}

impl fmt::Display for Arce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arce::Open => write!(f, "."),
            Arce::Tree => write!(f, "|"),
            Arce::Lumberyard => write!(f, "#"),
        }
    }
}