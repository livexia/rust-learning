use super::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // see https://en.wikipedia.org/wiki/Bubble_sort
        // repeatedly steps through the list,
        // compares adjacent elements and swaps them if they are in the wrong order.
        let mut swaped = true;
        while swaped {
            swaped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swaped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 6, 1, 2, 3];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}
