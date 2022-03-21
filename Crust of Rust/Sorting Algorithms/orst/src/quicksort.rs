use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    // if slice length is 0 or 1, then slice is sorted just return
    match slice.len() {
        0 | 1 => return,
        _ => (),
    }
    // choos pivot index with 0
    let pivot = 0;

    // left is start is 1
    // slice[(pivot + 1)..left] is all the value that smaller than the pivot
    // slice[left..] is all the value that bigger than the pivoit
    let mut left = 1;
    let mut right = slice.len() - 1;
    while left <= right {
        if slice[left] < slice[pivot] {
            // if slice[left] is smaller than the slice[pivot], then left should increase
            left += 1;
        } else if slice[right] > slice[pivot] {
            // if slice[right] is bigger than the slice[pivot]
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            // jsut to decrease the right to check another value
            right -= 1;
        } else {
            // if slice[left] is bigger than the slice[pivot]
            // then swap the slice at left an right,
            // because after swap slice[right] is bigger than the slice[pivot],
            // so we need decrease the right
            slice.swap(left, right);
            right -= 1;
        }
    }
    slice.swap(pivot, left - 1);
    quicksort(&mut slice[..left]);
    quicksort(&mut slice[left..]);
}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // see https://en.wikipedia.org/wiki/Quicksort
        // divide and conquer algorithm
        // It works by selecting a 'pivot' element from the array
        // and partitioning the other elements into two sub-arrays,
        // according to whether they are less than or greater than the pivot.

        quicksort(slice);
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 6, 1, 2, 3];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}

#[test]
fn empty() {
    let mut things: Vec<isize> = vec![];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[]);
}

#[test]
fn one() {
    let mut things = vec![1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1]);
}

#[test]
fn soretd() {
    let mut things = vec![1, 2, 3, 4, 5, 6];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5, 6]);
}
