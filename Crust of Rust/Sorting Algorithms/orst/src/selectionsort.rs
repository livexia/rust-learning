use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // see https://en.wikipedia.org/wiki/Selection_sort
        // [ sorted | unsorted ]
        // find the min value in the unsorted part
        // put the min value to the sorted part

        for unsorted in 0..slice.len() {
            let min_index = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| i + unsorted)
                // if slice is empty, then it won't go inside the for loop
                // so if inside the for loop, then the slice won't be empty
                .expect("slice is non-empty");
            slice.swap(unsorted, min_index);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 6, 1, 2, 3];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}
