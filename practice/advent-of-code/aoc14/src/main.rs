use std::io::{self, Write};
use std::error::Error;
use std::result;
use std::collections::HashMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;  
    
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let stop: usize = input.trim().parse()?;
    let (mut recipe1, mut recipe2) = (3, 7);
    let (mut start1, mut start2) = (0, 1);
    let mut recipes = vec![3, 7];
    let mut n = recipes.len();
    while n < stop + 11 {
        let sum = recipe1 + recipe2;
        update_recipes(&mut recipes, sum);
        n = recipes.len();
        let (step1, step2) = (recipe1 as usize + 1, recipe2 as usize + 1);
        start1 = (start1 + step1) % n;
        start2 = (start2 + step2) % n;
        recipe1 = recipes[start1];
        recipe2 = recipes[start2];
    }
    writeln!(
        io::stdout(), 
        "the scores of the ten recipes immediately after {} of recipes: {}", 
        stop, 
        recipes[stop..stop+10]
            .iter()
            .map(|n| n.to_string()).collect::<Vec<String>>()
            .join("")
    )?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let seuqence: Vec<u8> = input.trim().bytes().map(|c| c - b'0' as u8).collect();
    let m = seuqence.len();

    let (mut recipe1, mut recipe2) = (3, 7);
    let (mut start1, mut start2) = (0, 1);
    let mut recipes = vec![3, 7];

    let count;
    
    loop {
        let sum = recipe1 + recipe2;
        update_recipes(&mut recipes, sum);
        let n = recipes.len();
        let (step1, step2) = (recipe1 as usize + 1, recipe2 as usize + 1);
        start1 = (start1 + step1) % n;
        start2 = (start2 + step2) % n;
        recipe1 = recipes[start1];
        recipe2 = recipes[start2];
        if n > m && recipes[n-m..n] == seuqence[..] {
            count = n - m;
            break;
        }
        if sum >= 10 && n > m + 1 && recipes[n-m-1..n-1] == seuqence[..] {
            count = n - m - 1;
            break;
        }
    }

    writeln!(
        io::stdout(), 
        "{} recipes appear on the scoreboard to the left of the score sequence: {}", 
        count, &input)?;
    Ok(())
}

fn print_recipes(recipes: &Vec<u8>, start1: usize, start2: usize) {
    let n = recipes.len();
    for i in 0..n {
        if i == start1 {
            print!("({}) ", recipes[i]);
        } else if i == start2 {
            print!("[{}] ", recipes[i]);
        } else {
            print!(" {}  ", recipes[i]);
        }
    }
    println!();
}

fn update_recipes(recipes: &mut Vec<u8>, n: u8) {
    if n < 10 {
        recipes.push(n)
    } else {
        recipes.push(1);
        recipes.push(n - 10);
    }
}