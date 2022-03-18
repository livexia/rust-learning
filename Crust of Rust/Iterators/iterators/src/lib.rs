fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
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
    iter: O,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten { iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(flatten(::std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(vec![vec![1, 2, 3]]).count(), 3)
    }
}
