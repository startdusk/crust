use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// for bad test
// unsafe impl<T> Sync for Cell<T> {}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any references, because we give any out
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use std::sync::Arc;
    use std::thread;

    // #[test]
    // fn bad() {
    //     let x = Arc::new(Cell::new(0));
    //     let x1 = Arc::clone(&x);
    //     let th1 = thread::spawn(move || {
    //         for _ in 0..1000000 {
    //             let x = x1.get();
    //             x1.set(x + 1)
    //         }
    //     });

    //     let x2 = Arc::clone(&x);
    //     let th2 = thread::spawn(move || {
    //         for _ in 0..1000000 {
    //             let x = x2.get();
    //             x2.set(x + 1)
    //         }
    //     });

    //     th1.join().unwrap();
    //     th2.join().unwrap();

    //     assert_eq!(x.get(), 2000000);
    // }
}
