fn main() {
    println!("Hello, world!"); // println macro

    // equivalent
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello, world!\n").unwrap();

    // panic macro
    // panic!("This is a panic"); // thread 'main' panicked at 'This is a panic', src\main.rs:9:5

    // Option type can contain something, or it can contain nothing
    // If .unwrap() is called on it, and it contains nothing, it panics
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // fine
    let o2: Option<i32> = None;
    // o2.unwrap(); // will panic
}
