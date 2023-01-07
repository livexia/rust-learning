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
        let row = self.floor[x];
        row & (1 << y) != 0
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        let row = self.seat[x];
        row & (1 << y) != 0
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
                    let adjacent_seats = if part == 1 {
                        self.adjacent_seats(x, y)
                    } else if part == 2 {
                        self.first_seats(x, y)
                    } else {
                        unimplemented!("part: {part}")
                    };
                    if self.is_empty(x, y) {
                        if !adjacent_seats.iter().all(|&(x1, y1)| self.is_empty(x1, y1)) {
                            *row |= 1 << y;
                        }
                    } else if adjacent_seats
                        .iter()
                        .filter(|&&(x1, y1)| !self.is_empty(x1, y1))
                        .count()
                        >= threshold
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

    fn adjacent_seats(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![
            if let Some(up) = self.up_seat(x, y) {
                vec![
                    self.left_seat(up.0, up.1),
                    self.right_seat(up.0, up.1),
                    Some(up),
                ]
            } else {
                vec![]
            },
            if let Some(down) = self.down_seat(x, y) {
                vec![
                    self.left_seat(down.0, down.1),
                    self.right_seat(down.0, down.1),
                    Some(down),
                ]
            } else {
                vec![]
            },
            vec![self.left_seat(x, y), self.right_seat(x, y)],
        ]
        .iter()
        .flatten()
        .filter_map(|&p| p)
        .filter(|&(x, y)| !self.is_floor(x, y))
        .collect()
    }

    fn first_seats(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut seats = vec![];
        let (mut nx, mut ny) = (x, y);
        while let Some(up) = self.up_seat(nx, ny) {
            if self.is_floor(up.0, up.1) {
                (nx, ny) = up;
            } else {
                seats.push(up);
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(up) = self.up_seat(nx, ny) {
            if let Some(left) = self.left_seat(up.0, up.1) {
                if self.is_floor(left.0, left.1) {
                    (nx, ny) = left;
                } else {
                    seats.push(left);
                    break;
                }
            } else {
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(up) = self.up_seat(nx, ny) {
            if let Some(right) = self.right_seat(up.0, up.1) {
                if self.is_floor(right.0, right.1) {
                    (nx, ny) = right;
                } else {
                    seats.push(right);
                    break;
                }
            } else {
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(down) = self.down_seat(nx, ny) {
            if self.is_floor(down.0, down.1) {
                (nx, ny) = down;
            } else {
                seats.push(down);
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(down) = self.down_seat(nx, ny) {
            if let Some(left) = self.left_seat(down.0, down.1) {
                if self.is_floor(left.0, left.1) {
                    (nx, ny) = left;
                } else {
                    seats.push(left);
                    break;
                }
            } else {
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(down) = self.down_seat(nx, ny) {
            if let Some(right) = self.right_seat(down.0, down.1) {
                if self.is_floor(right.0, right.1) {
                    (nx, ny) = right;
                } else {
                    seats.push(right);
                    break;
                }
            } else {
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(left) = self.left_seat(nx, ny) {
            if self.is_floor(left.0, left.1) {
                (nx, ny) = left;
            } else {
                seats.push(left);
                break;
            }
        }
        let (mut nx, mut ny) = (x, y);
        while let Some(right) = self.right_seat(nx, ny) {
            if self.is_floor(right.0, right.1) {
                (nx, ny) = right;
            } else {
                seats.push(right);
                break;
            }
        }

        seats
    }

    fn up_seat(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x > 0 {
            Some((x - 1, y))
        } else {
            None
        }
    }

    fn down_seat(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x + 1 < self.height {
            Some((x + 1, y))
        } else {
            None
        }
    }

    fn left_seat(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y > 0 {
            Some((x, y - 1))
        } else {
            None
        }
    }

    fn right_seat(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y + 1 < self.width {
            Some((x, y + 1))
        } else {
            None
        }
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
    assert_eq!(vec![(1, 1), (1, 0)], grid.adjacent_seats(0, 0));
    assert_eq!(grid.is_empty(0, 0), true);
    assert_eq!(part1(&mut grid.clone()).unwrap(), 37);
    assert_eq!(part2(&mut grid).unwrap(), 26);
}
