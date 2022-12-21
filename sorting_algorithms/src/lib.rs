mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::Bubblesort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSortter;
    impl Sorter for StdSortter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSortter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
