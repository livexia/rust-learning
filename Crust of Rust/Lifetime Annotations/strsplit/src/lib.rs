
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(start) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..start];
                self.remainder = Some(&remainder[(start + self.delimiter.len())..]);
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}


pub fn until_char(s: &str, c: char) -> &str {
    let delimiter = format!("{}", c);
    StrSplit::new(s, &delimiter).next().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::{StrSplit, until_char};

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello, world", 'o'), "hell");
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d e f";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d e f ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f", ""]);
        assert_eq!(letters, haystack.split(" ").collect::<Vec<_>>());
    }
}
