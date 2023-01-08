use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let actions = parse_input(&input)?;

    part1(&actions)?;
    // part2()?;
    Ok(())
}

fn part1(actions: &[(u8, i32)]) -> Result<Int> {
    let start = Instant::now();

    let mut ship = Ship::new();
    for action in actions {
        ship.move_by_action(action);
    }
    let result = ship.distance();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn parse_input(input: &str) -> Result<Vec<(u8, Int)>> {
    let mut result = vec![];
    for line in input.lines() {
        let line = line.trim();
        if !line.is_empty() {
            let chars: Vec<_> = line.chars().collect();
            let action = if chars[0].is_alphabetic() {
                chars[0]
            } else {
                return err!("not a valid action: {line}");
            };
            let action = match action {
                'N' => 0,
                'E' => 1,
                'S' => 2,
                'W' => 3,
                'F' => 4,
                'L' => 5,
                'R' => 6,
                _ => return err!("not a valid action: {line}"),
            };
            let value: Int = chars[1..].iter().collect::<String>().parse()?;
            if (action == 5 || action == 6) && value % 90 != 0 {
                return err!("not a valid action: {line}, {:?}, {:?}", action, value);
            }
            result.push((action, value))
        }
    }
    Ok(result)
}

#[derive(Debug)]
struct Ship {
    facing: u8,           // N => 0, E => 1, S => 2, W => 3
    location: (Int, Int), // East, North
}

impl Ship {
    fn new() -> Self {
        Self {
            facing: 1,
            location: (0, 0),
        }
    }

    fn distance(&self) -> Int {
        self.location.0.abs() + self.location.1.abs()
    }

    fn move_by_action(&mut self, action: &(u8, Int)) {
        let value = action.1;
        match action.0 {
            4 => self.move_stright(value),
            0 => self.move_north(value),
            1 => self.move_east(value),
            2 => self.move_south(value),
            3 => self.move_west(value),
            5 | 6 => self.turn(action),
            _ => unreachable!(),
        }
    }

    fn turn(&mut self, action: &(u8, Int)) {
        let &(dir, value) = action;
        let value = ((value % 360) / 90) as u8;
        if dir == 5 {
            // turn left
            self.facing = (self.facing + 4 - value) % 4
        } else if dir == 6 {
            // turn right
            self.facing = (self.facing + value) % 4
        } else {
            unreachable!()
        }
    }

    fn move_stright(&mut self, value: Int) {
        match self.facing {
            0 => self.move_north(value),
            1 => self.move_east(value),
            2 => self.move_south(value),
            3 => self.move_west(value),
            _ => unreachable!(),
        }
    }

    fn move_east(&mut self, value: Int) {
        self.location.0 += value
    }

    fn move_north(&mut self, value: Int) {
        self.location.1 += value
    }

    fn move_west(&mut self, value: Int) {
        self.move_east(-value)
    }

    fn move_south(&mut self, value: Int) {
        self.move_north(-value)
    }
}

#[test]
fn example_input() {
    let input = "F10
    N3
    F7
    R90
    F11";
    let actions = parse_input(input).unwrap();
    assert_eq!(part1(&actions).unwrap(), 25);
}
