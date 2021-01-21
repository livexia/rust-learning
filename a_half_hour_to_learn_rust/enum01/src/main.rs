fn main() {
    use self::Option::{None, Some};
    let o1: Option<i32> = Some(128);
    o1.unwrap();
    
    // let o2: Option<i32> = None;
    // o2.unwrap(); // will panic
    
    println!("Hello, world!");
}

// Option is not a struct, it is an enum
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None Option")
        }
    }
}

// Result is also an enum
enum Result<T, E> {
    Ok(T),
    Err(E),
} // also panic when unwrapped and containing an error 