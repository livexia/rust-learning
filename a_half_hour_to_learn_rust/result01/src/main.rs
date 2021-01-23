fn main() -> Result<(), std::str::Utf8Error> {
    println!("Hello, world!");
    let s = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s); // Ok("🍉")

    let s = std::str::from_utf8(&[240, 159]);
    println!("{:?}", s); // Err(Utf8Error { valid_up_to: 0, error_len: None })

    // panic when failure
    // let s = std::str::from_utf8(&[195, 40]).unwrap(); // will panic
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Utf8Error { valid_up_to: 0, error_len: Some(1) }', src\main.rs:10:45

    // expect for a custom message
    // let s = std::str::from_utf8(&[240, 159]).expect("valid utf-8");
    // thread 'main' panicked at 'valid utf-8: Utf8Error { valid_up_to: 0, error_len: None }', src\main.rs:14:46

    // match result
    // match std::str::from_utf8(&[240, 159]) {
    //     Ok(s) => println!("{}", s),
    //     Err(e) => panic!(e), // thread 'main' panicked at 'Box<Any>', src\main.rs:20:19
    // }

    // if let
    if let Ok(s) = std::str::from_utf8(&[240, 159, 141, 137]) {
        println!("{}", s); // 🍉
    }

    // bubble up the error

    match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }
    // Ok(())

    // more concise
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    Ok(())
}
