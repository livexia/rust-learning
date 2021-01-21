fn main() {
    // variables have a lifetime
    println!("Hello, world!");
    // x doesn't exist yet
    {
        let x = 34; // x start existing
        println!("x = {}", x);
        // x stop existing
    }
    // x no longer exists

    // Reference also have a lifetime
    // x dones not exist yet
    {
        let x = 33; // x start existing
        let x_ref = &x; // x_ref is start existing, it borrows x
        println!("x_ref = {}", x_ref);
        // x stop existing
        // x_ref stop existing
    }
    // x no longer exists
    
    // ref lifetime can not exceed the lifetime of the variable binging it borrows
    // let  x_ref = {
    //     let x = 43;
    //     &x
    // };
    // println!("x_ref = {}", x_ref); // error: borrowed value does not live long enough

    // A variable binding can be immutably borrowed multiple times
    let x = 43;
    let x_ref1 = &x;
    let x_ref2 = &x;
    let x_ref3 = &x;
    println!("{} {} {}", x_ref1, x_ref2, x_ref3);

    // while borrowed, a variable binding can not be mutated
    // let mut x = 43;
    // let x_ref = &x;
    // x = 13;
    // println!("x_ref = {}", x_ref); // error[E0506]: cannot assign to `x` because it is borrowed

    // while immutably borrowed, a variable binding can not be mutated
    // let mut x = 43;
    // let x_ref1 = &x;
    // let x_ref2 = &mut x;
    // x = 13;
    // println!("x_ref1 = {}", x_ref1); // error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
}

// references in functiion arguments also have lifetimes
fn print(x: &i32) {
    // x is borrowed (from the callee) for the entire time of this function is called
}