use super::Sorter;

pub struct HeapSort;

fn heapify<T: Ord>(slice: &mut [T], root: usize) {
    // make sure the root has the biggest value
    // think of the slice as a level oreder traveresal of a binary tree
    // if x is the root, x * 2 + 1 is the left, x * 2 + 2 is the right
    let left = root * 2 + 1;
    let right = root * 2 + 2;
    let length = slice.len();
    let mut max = root;
    // find the biggest node
    if left < length && slice[max] < slice[left] {
        max = left
    }
    if right < length && slice[max] < slice[right] {
        max = right
    }
    if max != root {
        // if max value is not the root
        // swap the root value with the child
        slice.swap(root, max);
        // and make sure the affacted child tree is also heapify
        // heapify(slice, max);
    }
}

impl<T> Sorter<T> for HeapSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // see https://en.wikipedia.org/wiki/Heapsort
        // see https://www.geeksforgeeks.org/heap-sort/
        // see https://github.com/jonhoo/orst/blob/master/src/heapsort.rs

        // building a heap from the bottom up, heapify is shift down to establish the heap property
        // think of the slice as a level oreder traveresal of a binary tree
        for i in (0..(slice.len() / 2)).rev() {
            heapify(slice, i);
        }

        for unsorted in (0..slice.len()).rev() {
            slice.swap(unsorted, 0);

            heapify(&mut slice[..unsorted], 0);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 6, 1, 2, 3, 9, 10, 20, 100];
    let slice = &mut things;
    for i in (0..(slice.len() / 2)).rev() {
        heapify(slice, i);
    }
    eprintln!("{:?}", slice);

    let mut things = vec![5, 6, 1, 2, 3];
    HeapSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}
