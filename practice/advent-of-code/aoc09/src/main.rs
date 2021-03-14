use std::{env::{var, var_os}, fmt, io::{self, Read, Write}};
use std::error::Error;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<&str> = input.trim().split(' ').filter(|&x| {
        match x.parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }).collect();
    let max_player: usize = input[0].parse()?;
    let max_marble: usize = input[1].parse()?;
    println!("{}, {}", max_player, max_marble);

    part1(max_player, max_marble)?;
    part1_way2(max_player, max_marble)?;
    part2(max_player, max_marble * 100)?;
   
    Ok(())
}

fn get_index(circle_length: usize, cur: usize, step: usize, clockwise: bool) -> usize {
    if clockwise {
        if cur + step > circle_length {
            return cur + step - circle_length;
        }
        return cur + step;
    }
    if cur < step {
        return cur + circle_length - step;
    }
    cur - step
}

fn part1(max_player: usize, max_marble: usize) -> Result<()> {
    let mut cur_player = 1;
    let mut cur_index = 0;
    let mut circle: Vec<usize> = vec![0];
    let mut socre: Vec<usize> = vec![0; max_player];
    for cur_marble in 1..=max_marble {
        let index;
        if cur_marble % 23 == 0 {
            socre[cur_player] += cur_marble;
            index = get_index(circle.len(), cur_index, 7, false);
            socre[cur_player] += circle.remove(index);
        } else {
            index = get_index(circle.len(), cur_index, 2, true);
            circle.insert(index, cur_marble);
        }
        cur_index = index;
        cur_player = (cur_player + 1) % max_player;
    }
    writeln!(io::stdout(), "winning score (part 1): {:?}", socre.iter().max().unwrap())?;
    Ok(())
}

fn part1_way2(max_player: usize, max_marble: usize) -> Result<()> {
    let mut players = vec![Player::default(); max_player];
    play_game(&mut players, &mut Circle::new(), max_marble as u32);
    writeln!(
        io::stdout(), 
        "winning score (part 1): {:?}", 
        players.iter().map(|p| p.points).max().unwrap()
    )?;
    Ok(())
}

fn part2(max_player: usize, max_marble: usize) -> Result<()> {
    let mut players = vec![Player::default(); max_player];
    play_game(&mut players, &mut Circle::new(), max_marble as u32);
    writeln!(
        io::stdout(), 
        "winning score (part 2): {:?}", 
        players.iter().map(|p| p.points).max().unwrap()
    )?;
    Ok(())
}

fn play_game(players: &mut [Player], circle: &mut Circle, marbles: u32) {
    let start = circle.max_marble() + 1;
    let end = start + marbles;
    for (player_id, value) in (0..players.len()).cycle().zip(start..end) {
        circle.turn(&mut players[player_id], value);
    }
}

#[derive(Clone, Debug, Default)]
struct Player {
    points: u32,
}

struct Marble {
    value: u32,
    prev: usize,
    next: usize,
}

struct Circle {
    marbles: Vec<Marble>,
    current: usize,
}

impl Circle {
    fn new() -> Self {
        let first = Marble { value: 0, prev: 0, next: 0 };
        Self {
            marbles: vec![first],
            current: 0
        }
    }

    fn turn(&mut self, player: &mut Player, value: u32) {
        let marble_id = self.add_marble(value);
        if value %23 !=  0 {
            let insert_at = self.clockwise(1);
            self.insert_after(marble_id, insert_at);
            self.current = marble_id;
        } else {
            player.points += value;
            let remove_id = self.counter_clockwise(7);
            player.points += self.marbles[remove_id].value;
            self.remove(remove_id);
            self.current = self.counter_clockwise(6);
        }
    }

    fn max_marble(&self) -> u32 {
        (self.marbles.len() - 1) as u32
    }

    fn add_marble(&mut self, value: u32) -> usize {
        let id = self.marbles.len();
        self.marbles.push(Marble { value, prev: 0, next: 0});
        id
    }

    fn insert_after(&mut self, to_insert: usize, after: usize) {
        let old_next = self.marbles[after].next;
        self.marbles[after].next = to_insert;
        self.marbles[old_next].prev = to_insert;
        self.marbles[to_insert].prev = after;
        self.marbles[to_insert].next = old_next;
    }

    fn remove(&mut self, id: usize, ) {
        let (prev, next) = (self.marbles[id].prev, self.marbles[id].next);
        self.marbles[prev].next = next;
        self.marbles[next].prev = prev;
    }

    fn clockwise(&mut self, mut i: usize) -> usize {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].next;
            i -= 1;
        }
        id
    }

    fn counter_clockwise(&mut self, mut i: usize) -> usize {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].prev;
            i -= 1;
        }
        id
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut id = self.current;
        loop {
            let m = &self.marbles[id];
            write!(f, "{}", m.value)?;
            id = m.next;
            if id == self.current {
                break;
            }
        }
        Ok(())
    }
}
