fn main() {
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("positive: {}", minus_two.is_strictly_positive());

    // variable is immutable by default, so struct interior can not be mutated
    // n.odd = true; // error

    // can not be addigned   
    let n = Number {
        odd: false,
        value: 4,
    };
    // error, can not be addigned to 
    // n = Number {
    //     odd: true,
    //     value: 10,
    // }

    // use mut can make a variable mutable
    let mut n = Number {
        odd: true,
        value: 17,
    };
    n.value = 21; // No error
}

// define method on own types
struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}