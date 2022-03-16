pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((start, end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..start];
                self.remainder = Some(&remainder[end..]);
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .position(|(_, c)| &c == self)
            // .map(|start| (start, start + 1))
            .map(|start| (start, start + self.len_utf8()))   // use len_utf_8() to make sure use right char length
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().unwrap()
}

pub fn until_string(s: &str, p: String) -> &str {
    StrSplit::new(s, &p[..]).next().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{until_char, until_string, StrSplit};

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello, world", 'o'), "hell");
    }

    #[test]
    fn until_utf8char_test() {
        assert_eq!(until_char("hello😀, world", '😀'), "hello");
    }

    #[test]
    fn until_string_test() {
        assert_eq!(until_string("hello, world", 'o'.to_string()), "hell");
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
