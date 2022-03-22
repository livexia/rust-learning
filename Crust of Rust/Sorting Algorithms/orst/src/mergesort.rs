use super::Sorter;

pub struct MergeSort;

fn merge_sort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        _ => (),
    }
    let mid = slice.len() / 2;
    // let (left, right) = slice.split_at_mut(mid);
    merge_sort(&mut slice[..mid]);
    merge_sort(&mut slice[mid..]);
    merge(slice, mid);
}

fn merge<T: Ord>(slice: &mut [T], mid: usize) {
    let mut i = 0;
    let mut j = mid;
    while i < j && j < slice.len() {
        if slice[i] > slice[j] {
            j += 1;
            slice[i..j].rotate_right(1);
        }
        i += 1;
    }
}

impl<T> Sorter<T> for MergeSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // see https://en.wikipedia.org/wiki/Merge_sort
        // Divide the unsorted list into n sublists,
        // each containing one element (a list of one element is considered sorted).
        // Repeatedly merge sublists to produce new sorted sublists until there is only one sublist remaining.
        // This will be the sorted list.
        merge_sort(slice)
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 6, 1, 2, 3];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}

#[test]
fn empty() {
    let mut things: Vec<isize> = vec![];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[]);
}

#[test]
fn one() {
    let mut things = vec![1];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1]);
}

#[test]
fn soretd() {
    let mut things = vec![1, 2, 3, 4, 5, 6];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5, 6]);
}
