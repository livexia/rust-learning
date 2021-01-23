fn main() {
    println!("Hello, world!");

    let v = vec![1, 2, 3, 4, 5];

    let v2 = &v[2..4];

    println!("{:?}", v2);

    // range syntax is ..
    println!("{:?}", (0..).contains(&100)); // true
    println!("{:?}", (0..20).contains(&20)); // false
    println!("{:?}", (0..=20).contains(&20)); // true
    println!("{:?}", (3..6).contains(&4)); // true

    // legal
    let y = {
        let x = &[1, 2, 3, 4, 99];
        tail(x)
    };
    println!("y: {:?}", y);

    // illeagl borrowed value does not live long enough
    // let y = {
    //     let x = vec![1, 2, 3, 5, 10];
    //     tail(&x) // borrowed value does not live long enough
    // };
    // println!("y: {:?}", y);

}

fn tail<'a>(s: &'a [u8]) -> &'a [u8] {
    &s[1..] 
}

