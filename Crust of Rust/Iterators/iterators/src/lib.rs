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
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer_iter: iter,
            front_iter: None,
            back_iter: None,
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
            if let Some(ref mut inner_iter) = self.front_iter {
                if let Some(inner_item) = inner_iter.next() {
                    return Some(inner_item);
                }
                self.front_iter = None;
            }
            if let Some(front_iter) = self.outer_iter.next() {
                self.front_iter = Some(front_iter.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
            // self.next() // use recursion or loop
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.back_iter {
                if let Some(inner_item) = inner_iter.next_back() {
                    return Some(inner_item);
                }
                self.back_iter = None;
            }
            if let Some(back_iter) = self.outer_iter.next_back() {
                self.back_iter = Some(back_iter.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
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

    #[test]
    fn inf() {
        let mut i = flatten((0..).map(|x| 0..x));
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
    }

    #[test]
    fn front() {
        let mut i = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next(), Some(5));
        assert_eq!(i.next(), Some(6));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn back() {
        let mut i = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(i.next_back(), Some(6));
        assert_eq!(i.next_back(), Some(5));
        assert_eq!(i.next_back(), Some(4));
        assert_eq!(i.next_back(), Some(3));
        assert_eq!(i.next_back(), Some(2));
        assert_eq!(i.next_back(), Some(1));
        assert_eq!(i.next_back(), None);
    }

    #[test]
    fn two_way() {
        let mut i = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(i.next_back(), Some(6));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next_back(), Some(5));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next_back(), Some(4));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next_back(), None);
        assert_eq!(i.next(), None);
    }

    #[test]
    fn rev() {
        assert_eq!(
            flatten(vec![vec![1, 2, 3]]).rev().collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn rev_wide() {
        assert_eq!(
            flatten(vec![vec![1, 2, 3], vec![4, 5, 6]])
                .rev()
                .collect::<Vec<_>>(),
            vec![6, 5, 4, 3, 2, 1]
        );
    }
}
