pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer_iter: O,
    inner_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer_iter: iter,
            inner_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner_iter {
                if let Some(inner_item) = inner_iter.next() {
                    return Some(inner_item);
                }
                self.inner_iter = None;
            }
            self.inner_iter = Some(self.outer_iter.next()?.into_iter());
            // self.next() // use recursion or loop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(::std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn once() {
        assert_eq!(flatten(::std::iter::once(vec!['1'])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(::std::iter::once(vec!['1', '2'])).count(), 2);
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(vec![vec![1, 2, 3]]).count(), 3);
    }
    #[test]
    fn wide() {
        assert_eq!(flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]).count(), 6);
    }
}
