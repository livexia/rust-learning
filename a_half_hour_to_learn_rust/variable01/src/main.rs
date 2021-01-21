fn main() {
    // let

    let x; // declare
    x = 55;
    println!("{}", x);

    //same as
    let x = 55;
    println!("{}", x);

    // declare with type
    let x: i32 = 55;
    println!("i32 x: {}", x);

    // can not x = x + 3;
    // but can do let x = x + 3;
    let x = x + 3;
    println!("{}", x);

    // tuple
    let pair = ('a', '5');
    println!("{}", pair.0);
    println!("{}", pair.1);

    // annotate the type of tuple
    let pair: (char, i32);
    pair = ('b', 6);
    println!("{}, {}", pair.0, pair.1);

    // destructured tuple
    let (pair1, pair2) = pair;
    println!("{}, {}", pair1, pair2);

    // when function return tuple
    let x:[i32; 6] = [0, 1, 2, 3, 4, 5];
    let (part1, part2) = x.split_at(3);
    println!("{:?}, {:?}", part1, part2);
    
    // throw away part, throw away less than 2 part
    let (_, part) = x.split_at(2);
    println!("{:?}", part);
}