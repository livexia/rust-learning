use std::io::{self, Read, Write};
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

fn part2(max_player: usize, max_marble: usize) -> Result<()> {
    writeln!(io::stdout(), "winning score (part 2): {:?}", todo!())?;
    todo!("timeout");
}