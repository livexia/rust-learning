use orst::*;

use rand::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}
impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    println!("algorithm n comparisions time");
    for &n in &[0, 10, 100, 1000, 10000, 50000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }
        for _ in 0..10 {
            values.shuffle(&mut rand); // use rand::prelude::*

            run_bench!(n, BubbleSort, "bubble", &values, &counter);
            run_bench!(
                n,
                InsertionSort { smart: false },
                "insertion-dumb",
                &values,
                &counter
            );
            run_bench!(
                n,
                InsertionSort { smart: true },
                "insertion-smart",
                &values,
                &counter
            );
            run_bench!(n, SelectionSort, "selection", &values, &counter);
            run_bench!(n, QuickSort, "quick", &values, &counter);
            run_bench!(n, HeapSort, "heap", &values, &counter);
            run_bench!(n, MergeSort, "merge", &values, &counter);
            run_bench!(n, StdSorter, "stdstable", &values, &counter);
            run_bench!(n, StdUnstableSorter, "stdunstable", &values, &counter);
        }
    }
}

#[macro_export]
macro_rules! run_bench {
    ($n: ident, $sorter: expr, $name: expr, $values: expr, $counter: expr) => {
        let took = bench($sorter, $values, $counter);
        println!("{} {} {} {}", $name, $n, took.0, took.1);
    };
}

fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1])
    }
    (count, took.as_secs_f64())
}
