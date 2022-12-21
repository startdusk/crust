use std::{cell::Cell, rc::Rc};

use rand::prelude::*;

use sorting_algorithms::*;

#[derive(Debug, Clone)]
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

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for &n in &[0, 1, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: thread_rng().gen::<usize>(),
                cmps: Rc::clone(&counter),
            })
        }
        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(Bubblesort, &values, &counter);
            println!("bubble {} {} {}", n, took.0, took.1);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("insertion-smart {} {} {}", n, took.0, took.1);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("insertion-dumb {} {} {}", n, took.0, took.1);
            let took = bench(SelectionSort, &values, &counter);
            println!("selection {} {} {}", n, took.0, took.1);
            let took = bench(QuickSort, &values, &counter);
            println!("quick {} {} {}", n, took.0, took.1);
        }
    }
}

fn bench<T: Ord + Clone, S: Sorter>(
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
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}
