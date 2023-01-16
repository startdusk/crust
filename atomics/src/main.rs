use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // x86: CAS
        // ARM: LDREX STREX
        //   - compare_exchange: impl using a loop of LDREX and STREX
        //   - compare_exchange_weak: LDREX STREX
        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MEST protocol: stay in S when locked
            while self.locked.load(Ordering::Relaxed) != LOCKED {
                std::thread::yield_now();
            }

            std::thread::yield_now();
        }
        // Safety: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

use std::thread::spawn;

#[test]
fn mutex_test() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handlers: Vec<_> = (0..10)
        .map(|_| {
            spawn(move || {
                for _ in 0..100 {
                    l.with_lock(|v| *v += 1);
                }
            })
        })
        .collect();

    for handle in handlers {
        handle.join().unwrap();
    }

    assert_eq!(l.with_lock(|v| *v), 10 * 100)
}

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(47, Ordering::Relaxed);
        r2
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
}

fn main() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    spawn(move || x.store(true, Ordering::Release));
    spawn(move || y.store(true, Ordering::Release));

    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}

        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}

        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
    // What are the possible values for z?
    //  - Is 0 possible?
    //  - Is 1 possible?
    //  - Is 2 possible?

}
