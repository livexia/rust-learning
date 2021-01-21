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