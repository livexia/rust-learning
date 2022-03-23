// signature 1
// pub fn strtok(s: &mut &str, delimiter: char) -> &str {

// signature 2
// pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
// when there all the life time is 'a,
// because with type &'a mut T, T is invariant
// the test case s type is &'static str
// so the 'a must be 'static
// then the s type is &'static mut &'static str,
// then in the test the &mut s will live as long as the program,
// then there are both a immutable reference and a mutable reference,
// this is not allowed

// signature 3
// pub fn strtok<'a, 's>(s: &'a mut &'s str, delimiter: char) -> &'s str {

// signature 4
pub fn strtok<'s>(s: &'_ mut &'s str, delimiter: char) -> &'s str {
    // because with type &'a mut T,
    // the 'a is covariant, T is invariant
    // the test case s type must be &'static str
    // beacuse T is invarinat, so the 's must be 'static
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + 1)..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::strtok;

    #[test]
    fn it_works() {
        let mut s = "hello world";
        let hello = strtok(&mut s, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(s, "world");
    }
}

// contravarinat

// fn test(f: fn(&'static str) -> ()) {
//     let s = "hello, world";
//     f(s);
// }

// invariant
// fn foo<'a>(s: &mut &'a str, x: &'a str) {
//     *s = x;
// }

// #[test]
// fn test_main() {
//     let mut s: &'static str = "hello world";
//     let x = String::new();
//     foo(&mut s, &x);
//     drop(x);
//     println!("{}", s);
//     // dbg!(x2);
// }

// why in &'a mut T 'a is covariant
pub fn bar() {
    let mut y = true;
    let mut z = &mut y;

    let x = Box::new(true);
    let x: &'static mut bool = Box::leak(x);
    let _ = z;
    z = x; // &'y mut bool = &'static mut bool
    drop(z)
}
