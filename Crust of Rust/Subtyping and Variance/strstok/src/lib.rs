pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
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
        let hello = strstok(&mut s, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(s, "world");
    }
}
