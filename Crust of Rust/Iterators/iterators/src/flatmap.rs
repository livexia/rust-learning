use std::iter::Map;

pub trait FlatMapExt: Iterator + Sized {
    fn our_flat_map<F, U>(self, f: F) -> FlatMap<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator;
}

impl<T> FlatMapExt for T
where
    T: Iterator,
{
    fn our_flat_map<F, U>(self, f: F) -> FlatMap<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        FlatMap::new(self, f)
    }
}

pub fn flat_map<I, F, U>(iter: I, f: F) -> FlatMap<I, F, U>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    FlatMap::new(iter, f)
}

pub struct FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    outer_iter: Map<O, F>,
    front_iter: Option<U::IntoIter>, // should be a iterator with Item is U
    back_iter: Option<U::IntoIter>,  // should be a iterator with Item is U
}

impl<O, F, U> FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    pub fn new(iter: O, f: F) -> Self {
        Self {
            outer_iter: iter.map(f),
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O, F, U> Iterator for FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.front_iter {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.front_iter = None;
            }
            if let Some(outer_item) = self.outer_iter.next() {
                self.front_iter = Some(outer_item.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            };
        }
    }
}

impl<O, F, U> DoubleEndedIterator for FlatMap<O, F, U>
where
    O: DoubleEndedIterator,
    F: FnMut(O::Item) -> U,
    U: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.back_iter {
                if let Some(item) = inner_iter.next_back() {
                    return Some(item);
                }
                self.back_iter = None;
            }
            if let Some(outer_item) = self.outer_iter.next_back() {
                self.back_iter = Some(outer_item.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            };
        }
    }
}

#[cfg(test)]
mod flatmap_test {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            flat_map(vec![].into_iter(), |x: Vec<i32>| x.into_iter()).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(
            flat_map(vec![vec![1]].into_iter(), |x| x.into_iter()).count(),
            1
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            flat_map(vec![vec![1, 2]].into_iter(), |x| x
                .into_iter()
                .map(|i| i + 1))
            .collect::<Vec<_>>(),
            vec![2, 3]
        );
    }

    #[test]
    fn deep() {
        assert_eq!(
            flat_map(vec![vec![1], vec![2]].into_iter(), |x| x
                .into_iter()
                .map(|i| i + 1))
            .collect::<Vec<_>>(),
            vec![2, 3]
        );
    }

    #[test]
    fn two_deep() {
        assert_eq!(
            flat_map(vec![vec![1, 2], vec![3, 4]].into_iter(), |x| x
                .into_iter()
                .map(|i| i + 1))
            .collect::<Vec<_>>(),
            vec![2, 3, 4, 5]
        );
    }

    #[test]
    fn merge_word() {
        let merged: String = flat_map(vec!["Hello", "World"].into_iter(), |x| x.chars()).collect();
        assert_eq!(&merged, "HelloWorld");
    }

    #[test]
    fn rev_inner() {
        let sub_sum: Vec<i32> = flat_map(vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter(), |x| {
            x.into_iter().rev().collect::<Vec<i32>>()
        })
        .collect();
        assert_eq!(sub_sum, vec![3, 2, 1, 6, 5, 4]);
    }

    #[test]
    fn two_deep_ext() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4]]
                .into_iter()
                .our_flat_map(|x| x.into_iter().map(|i| i + 1))
                .collect::<Vec<_>>(),
            vec![2, 3, 4, 5]
        );
    }

    #[test]
    fn merge_word_ext() {
        let merged: String = vec!["Hello", "World"]
            .into_iter()
            .our_flat_map(|x| x.chars())
            .collect();
        assert_eq!(&merged, "HelloWorld");
    }

    #[test]
    fn rev_inner_ext() {
        let sub_sum: Vec<i32> = vec![vec![1, 2, 3], vec![4, 5, 6]]
            .into_iter()
            .our_flat_map(|x| x.into_iter().rev().collect::<Vec<i32>>())
            .collect();
        assert_eq!(sub_sum, vec![3, 2, 1, 6, 5, 4]);
    }

    #[test]
    fn two_way_flatmap() {
        let mut i = vec![vec![1, 2, 3], vec![100, 200, 300]]
            .into_iter()
            .our_flat_map(|x| x.into_iter().map(|y| y + 1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next_back(), Some(301));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next_back(), Some(201));
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next_back(), Some(101));
        assert_eq!(i.next(), None);
        assert_eq!(i.next_back(), None);
    }

    #[test]
    fn compare_to_flat_map() {
        let words = ["alpha", "beta", "gamma"];
        let i = words.into_iter().our_flat_map(|s| s.chars());
        let std_flat_map = words.into_iter().flat_map(|s| s.chars());
        assert!(i.eq(std_flat_map));
    }

    #[test]
    fn compare_to_map_flatten() {
        let words = ["alpha", "beta", "gamma"];
        let i = words.into_iter().our_flat_map(|s| s.chars());
        let std_map_flat = words.into_iter().map(|s| s.chars()).flatten();
        assert!(i.eq(std_map_flat));
    }
}
