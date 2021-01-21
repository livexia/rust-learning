// marker trait
// not a type method
// certain things can be done with a type
fn main() {
    let a: i32 = 15;
    let b = a;
    let c = a;

    // i32 is Copy
    println!("{}, {}, {}", a, b, c);
    print_i32(a);
    print_i32(a);

    // Number is not Copy
    let n = Number { odd: true, value: 51 };
    let m = n;  // n is moved into m
    // let o = n;  // error: value used here after move

    print_number(m); // m is moved
    // print_number(m); // error: value used here after move
    
    let n = Number { odd: true, value: 71 };
    print_number_ref(&n);   // n is borrowed
    print_number_ref(&n);   // n is borrowed again

    // if you deine a variable by mut
    // you can do this:
    let mut n = Number { odd: false, value: 80 };
    print_number_ref(&n);
    invert(&mut n);     // n is brorrowed mutably
    print_number_ref(&n);

    // trait methods can also take self by reference or mutable reference
    let n = Number { odd: true, value: 99 };
    // let mut  m = n; //  n is moved
    let mut  m = n.clone(); 
    // equivalent:
    // let m = std::clone::Clone::clone(n);
    m.value += 100;
    print_number_ref(&n);
    print_number_ref(&m);


    // impl marker trait Copy
    let n = Number { odd: true, value: 55 };
    let m = n.clone();
    let o = n.clone();

    print_number(m);
    print_number(o);

    let m = n; // m is a copy of n
    let o = n; // o is a copy of n

    print_number(m);
    print_number(o);

    // common tarit, using derive attribute
}

fn print_i32(x: i32) {
    println!("{}", x);
}

// #[derive(Clone, Copy)]   // common tarit, using derive attribute
struct Number {
    odd: bool,
    value: i32,
    
}

fn print_number(n: Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

// immutable reference
fn print_number_ref(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

// mutable reference
fn invert(n: &mut Number) {
    n.value = -n.value;
}

// trait methods can also take self by reference or mutable reference
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

// Marker traits like Copy have no methods:
// note: `Copy` requires that `Clone` is implemented too
impl std::marker::Copy for Number {}
