use std::error::Error;
use std::fmt::Display;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut grid: Grid = input.parse()?;

    part1(&mut grid.clone())?;
    part2(&mut grid)?;
    Ok(())
}

fn part1(grid: &mut Grid) -> Result<usize> {
    let start = Instant::now();

    while grid.next_round(4, 1) {}
    let result = grid.occupied_seat_counter();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(grid: &mut Grid) -> Result<usize> {
    let start = Instant::now();

    while grid.next_round(5, 2) {}
    let result = grid.occupied_seat_counter();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

#[derive(Clone)]
struct Grid {
    floor: Vec<u128>, // bit with 1 mean floor
    seat: Vec<u128>,  // bit with 0 mean occupied, 1 mean empty
    height: usize,
    width: usize,
}

impl Grid {
    fn is_floor(&self, x: usize, y: usize) -> bool {
        self.floor[x] & (1 << y) != 0
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.seat[x] & (1 << y) != 0
    }

    fn occupied_seat_counter(&self) -> usize {
        let mut counter = 0;
        for x in 0..self.height {
            for y in 0..self.width {
                if !self.is_floor(x, y) && !self.is_empty(x, y) {
                    counter += 1
                }
            }
        }
        counter
    }

    fn next_round(&mut self, threshold: usize, part: usize) -> bool {
        let mut seat = vec![0; self.height];
        for (x, row) in seat.iter_mut().enumerate() {
            for y in 0..self.width {
                if !self.is_floor(x, y) {
                    let occupied_counter = if part == 1 {
                        self.adjacent(x, y)
                            .iter()
                            .filter(|&&(x1, y1)| !self.is_floor(x1, y1) && !self.is_empty(x1, y1))
                            .count()
                    } else if part == 2 {
                        self.visible_seats(x, y)
                            .iter()
                            .filter(|&&(x1, y1)| !self.is_empty(x1, y1))
                            .count()
                    } else {
                        unimplemented!("part: {part}")
                    };
                    if self.is_empty(x, y) && occupied_counter != 0 || occupied_counter >= threshold
                    {
                        *row |= 1 << y;
                    }
                }
            }
        }
        if self.seat == seat {
            return false;
        }
        self.seat = seat;
        true
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut seats = Vec::with_capacity(8);
        if let Some(next) = up(self, x, y) {
            seats.push(next);
            if let Some(next) = left(self, next.0, next.1) {
                seats.push(next);
            }
            if let Some(next) = right(self, next.0, next.1) {
                seats.push(next);
            }
        }
        if let Some(next) = down(self, x, y) {
            seats.push(next);
            if let Some(next) = left(self, next.0, next.1) {
                seats.push(next);
            }
            if let Some(next) = right(self, next.0, next.1) {
                seats.push(next);
            }
        }
        if let Some(next) = left(self, x, y) {
            seats.push(next);
        }
        if let Some(next) = right(self, x, y) {
            seats.push(next);
        }

        seats
    }

    fn visible_seats(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut seats = Vec::with_capacity(8);
        visible_stright(self, x, y, &mut seats, up);
        visible_stright(self, x, y, &mut seats, down);
        visible_stright(self, x, y, &mut seats, left);
        visible_stright(self, x, y, &mut seats, right);

        visible_diagonal(self, x, y, &mut seats, up, left);
        visible_diagonal(self, x, y, &mut seats, up, right);

        visible_diagonal(self, x, y, &mut seats, down, left);
        visible_diagonal(self, x, y, &mut seats, down, right);

        seats
    }
}

fn visible_stright(
    grid: &Grid,
    x: usize,
    y: usize,
    seats: &mut Vec<(usize, usize)>,
    f: fn(&Grid, x: usize, y: usize) -> Option<(usize, usize)>,
) {
    let (mut nx, mut ny) = (x, y);
    while let Some(next) = f(grid, nx, ny) {
        if grid.is_floor(next.0, next.1) {
            (nx, ny) = next;
        } else {
            seats.push(next);
            break;
        }
    }
}

fn visible_diagonal(
    grid: &Grid,
    x: usize,
    y: usize,
    seats: &mut Vec<(usize, usize)>,
    f1: fn(&Grid, x: usize, y: usize) -> Option<(usize, usize)>,
    f2: fn(&Grid, x: usize, y: usize) -> Option<(usize, usize)>,
) {
    let (mut nx, mut ny) = (x, y);
    while let Some((x, y)) = f1(grid, nx, ny) {
        if let Some(next) = f2(grid, x, y) {
            if grid.is_floor(next.0, next.1) {
                (nx, ny) = next;
            } else {
                seats.push(next);
                break;
            }
        } else {
            break;
        }
    }
}

fn up(_grid: &Grid, x: usize, y: usize) -> Option<(usize, usize)> {
    if x > 0 {
        Some((x - 1, y))
    } else {
        None
    }
}

fn down(grid: &Grid, x: usize, y: usize) -> Option<(usize, usize)> {
    if x + 1 < grid.height {
        Some((x + 1, y))
    } else {
        None
    }
}

fn left(_grid: &Grid, x: usize, y: usize) -> Option<(usize, usize)> {
    if y > 0 {
        Some((x, y - 1))
    } else {
        None
    }
}

fn right(grid: &Grid, x: usize, y: usize) -> Option<(usize, usize)> {
    if y + 1 < grid.width {
        Some((x, y + 1))
    } else {
        None
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut floor = vec![];
        let mut seat = vec![];
        let mut counter = 0;
        let mut width = 0;
        for line in s.lines() {
            let line = line.trim();
            width = line.len();
            if !line.is_empty() {
                floor.push(0);
                seat.push(0);
                for (i, c) in line.char_indices() {
                    match c {
                        '.' => floor[counter] |= 1 << i,
                        'L' => seat[counter] |= 1 << i,
                        _ => return err!("invalid grid"),
                    }
                }
                counter += 1;
            }
        }
        Ok(Grid {
            floor,
            seat,
            height: counter,
            width,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.height {
            for y in 0..self.width {
                if self.is_floor(x, y) {
                    write!(f, ".")?;
                } else if self.is_empty(x, y) {
                    write!(f, "L")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn example_input() {
    let input = "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";
    let mut grid: Grid = input.parse().unwrap();
    assert_eq!(grid.is_floor(0, 1), true);
    assert_eq!(grid.is_floor(2, 3), true);
    assert_eq!(grid.is_floor(3, 3), false);
    // assert_eq!(vec![(1, 1), (1, 0)], grid.adjacent_seats(0, 0));
    assert_eq!(grid.is_empty(0, 0), true);
    assert_eq!(part1(&mut grid.clone()).unwrap(), 37);
    assert_eq!(part2(&mut grid).unwrap(), 26);
}
