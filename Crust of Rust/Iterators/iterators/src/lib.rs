fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
{
    iter: O,
}

impl<O> Flatten<O>
where
    O: Iterator,
{
    pub fn new(iter: O) -> Self {
        Flatten { iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
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
}
