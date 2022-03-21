pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;
impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![5, 6, 1, 2, 3];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 5, 6]);
    }

    #[test]
    fn std_unstable_works() {
        let mut things = vec![5, 6, 1, 2, 3];
        StdUnstableSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 5, 6]);
    }
}
