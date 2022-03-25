use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Relaxed, Release, SeqCst};

const UNLOCKED: bool = false;
const LOCKED: bool = true;

pub struct Mutex<T> {
    lock: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            lock: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(value),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // while self.lock.load(Ordering::Relaxed) != UNLOCKED {}
        // self.lock.store(LOCKED, Ordering::Relaxed);
        // with load and store two atomic operation,
        // other therad could run between those two atomic operation

        // with compare_exchange this will run way slow,
        // because every loop check acquire access the unshared memory to try to write the new value
        // but there is much less chance that the test will failed
        while self
            .lock
            .compare_exchange(UNLOCKED, LOCKED, Relaxed, Relaxed)
            .is_err()
        {}
        let ret = f(unsafe { &mut *self.v.get() });
        self.lock.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
#[should_panic]
fn test_load_and_store() {
    use std::thread::spawn;
    // use Box to create a sendable data
    let m: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles: Vec<_> = (0..1000)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    m.with_lock(|v| *v += 1)
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    assert_ne!(m.with_lock(|v| *v), 1000 * 1000);
}
