use std::fmt::Display;
use std::fmt::Debug;

// generic function
fn main() {
    println!("Hello, world!");
    compare("tea", "coffee");
    compare("tea", "tea");

    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
}


// functions can be generic
fn foobar<T>(arg: T) {
    //do things with arg
}

// multiple type parameters
fn foobar_1<L, R>(left: L, right: R) {
    // do things with left, right
}


// type parameters usually have constraints
// trait name constraints
fn print<T: Display>(value: T) {
    println!("value = {}", value);
}

fn print_dbeug<T: Debug>(value: T) {
    println!("value = {:?}", value);
}

// longer version
fn print_longer<T>(value: T) 
where
    T: Display,
{
    println!("value = {}", value);
}

// complicated constraints

fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}
