pub fn strstok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
    ""
}

#[cfg(test)]
mod tests {
    use super::strstok;

    #[test]
    fn it_works() {
        let mut s = "hello world";
        let hello = strstok(&mut s, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(s, "world");
    }
}
