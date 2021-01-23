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
    
    // loop over vec
    for i in vec![52, 49, 21] {
        println!("NUmber: {}", i)
    }

    // loop over slice
    for i in &[22, 11, 45] {
        println!("NUmber: {}", i)
    }

    // loop over iterator
    // note: `&str` also has a `.bytes()` iterator.
    // Rust's `char` type is a "Unicode scalar value"
    for c in "rust".chars() {
        println!("char: {}", c)
    }

    // even if the iterator items are filtered and mapped and flattened
    for c in "sHE'S brOKen"
        .chars()
        .filter(|c| c.is_uppercase() || !c.is_ascii_alphabetic()) // 过滤掉大写字母和非ascii码
        .flat_map(|c| c.to_lowercase()) // 每个字符转为小写
    {
        print!("{}", c);
    }
    println!();
}

fn tail<'a>(s: &'a [u8]) -> &'a [u8] {
    &s[1..] 
}

