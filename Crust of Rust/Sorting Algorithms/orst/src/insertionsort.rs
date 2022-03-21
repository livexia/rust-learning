use super::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | unsorted ]
        // select first element on unsorted, find the location where the element fit
        // inster that element on the location

        if !self.smart {
            // unsorted pointer move backwards untill find location
            for unsorted in 1..slice.len() {
                let mut unsorted = unsorted;
                while unsorted > 0 && slice[unsorted - 1] > slice[unsorted] {
                    slice.swap(unsorted, unsorted - 1);
                    unsorted -= 1;
                }
            }
        } else {
            // use slice.binary_search to find the correct location
            // and use rotate_right to save swap times
            for unsorted in 1..slice.len() {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    // see https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
                    // If the value is found then Result::Ok is returned, containing the index of the matching element.
                    Ok(i) => i,
                    // If the value is not found then Result::Err is returned,
                    // containing the index where a matching element could be inserted while maintaining sorted order.
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_dumb() {
    let mut things = vec![5, 6, 1, 2, 3];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}

#[test]
fn it_works_smart() {
    let mut things = vec![5, 6, 1, 2, 3];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 5, 6]);
}
