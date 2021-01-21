fn main() {
    // semi-colom marks the end of a statement
    let x = 3;
    let y = 5;
    let z = x + y;

    // statements can span multiple lines
    let x = 
    10;
    println!("z= {}, x = {}", z, x);

    // void function
    great();

    // functinon with a return value
    println!("{}", return_five());

    // equivalent to this:
    println!("{}", return_five_without_return());

    // block
    let x = "out";
    {
        // different block, has own scope
        let x = "in";
        println!("{}", x); // int
    }
    println!("{}", x); // out

    // blocks are expressions
    let x = 42;
    // equivalent to this:
    let x = { 42 };
    println!("{}", x);

    // a block can be mutiple statement
    let x = {
        let y = 1;
        let z = 10;
        z - y
    };
    println!("x = {}", x);

    // if is also an expression
    let x = {
        if 1 < 2 {
            2
        } else {
            1
        }
    };
    println!("1<2 so x = {}", x);

    // match is also an expression
    let answer = true;
    let x = {
        match answer {
            true => 1,
            false => 0
        }
    };
    println!("because answer is {}, so x = {}", answer, x);

    // dot used to access fields of a value
    let a = (10, 20);
    let t = a.1;
    println!("{}", t);

    // dot used to call a method on a value
    let nick = "fasterthanlime";
    println!("nike length: {}", nick.len());

    // double-colon: operates on namespaces
    let least = std::cmp::min(3, 8);
    println!("least: {}", least);
    // is equivalent to this
    use std::cmp::min;
    let least = min(1, -10);
    println!("least: {}", least);

    // use directives, impoer both min and max
    // this works
    // use std::cmp::min;
    //use std::cmp::msx;

    // this also works
    // use std::cmp::{min, max};

    // this also works!
    // use std::{cmp::{min, max}};

    // use * to import every symbol from a namespace
    // use std::cmp::*;

    //Types are also namespaces too, methos can be called as regular functions
    let x = "amos".len(); //this is 4
    println!("x = {}", x);
    // use type namespace to call funtion
    let x = str::len("amos"); // this is also 4
    println!("x = {}", x);

    // str is a primitive type, there is also many non-primitive types are also in scope by default
    let v: Vec<i32> = Vec::new();

    // this is same, but with *full* path
    let v: Vec<i32> = std::vec::Vec::new();
    
    // this works because Rust inserts this at beginning of every modules
    // use std::prelude::v1::*;
}

fn great() {
    println!("Hi there!");
}

fn return_five() -> i32 {
    return 5;
}

// return five withou return 
fn return_five_without_return() -> i32 {
    5
}