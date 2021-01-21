struct Number {
    odd: bool,
    value: i32,
}
fn main() {
    // let patterns used as conditions in if
    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };
    print_number_if(one);
    print_number_if(two);


    // match arms also patterns
    let three = Number { odd: true, value: 3 };
    let four = Number { odd: false, value: 4 };
    print_number_match(three);
    print_number_match(four);


    // more match
    // match need exhaustive, at least one arm needs to match
    let five = Number { odd: true, value: 5 };
    let fourteen = Number { odd: false, value: 14 };
    print_number_other_than_five(five);
    print_number_other_than_five(fourteen);

    // exact same things
    let five = Number { odd: true, value: 5 };
    let fourteen = Number { odd: false, value: 14 };
    print_number_other_than_five2(five);
    print_number_other_than_five2(fourteen);
}

fn print_number_if(n: Number) {
    if let Number { odd: true, value } = n {
        println!("odd number: {}", value);
    } else if let Number { odd: false, value} = n {
        println!("even number: {}", value)
    }
}

fn print_number_match(n: Number) {
    match n {
        Number { odd: true, value} => println!("odd number: {}", value),
        Number { odd: false, value} => println!("even number: {}", value),
    }
}

fn print_number_other_than_five(n: Number) {
    match n {
        Number { value: 5, .. } => println!("skip"),
        Number { value, .. } => println!("{}", value),
    }
}

fn print_number_other_than_five2(n: Number) {
    match n.value {
        5 => println!("skip"),
        _ => println!("{}", n.value),
    }
}
