use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Relaxed, Release, SeqCst};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::thread::spawn;

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
        // because every loop check acquire exclusive access the memory to try to write the new value,
        // there is much more coordination between CPUs,
        // MESI protocol see: https://en.wikipedia.org/wiki/MESI_protocol
        // but there is much less chance that the test will failed

        // use compare_exchange_weak to replace compare_exchange,
        // because on some platform this will gain performance

        // when success change oredering to Acquire
        while self
            .lock
            .compare_exchange_weak(UNLOCKED, LOCKED, Acquire, Relaxed)
            .is_err()
        {
            // add a layer of loops to prevent each attempt to gain exclusive access to memory
            // this will run much quicker now.
            while self.lock.load(Relaxed) == LOCKED {}
        }
        // Safety: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        // change ordering to Release to make sure all access before release the lock
        self.lock.store(UNLOCKED, Release);
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

#[test]
#[should_panic]
fn too_relaxed() {
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        let r1 = y.load(Relaxed); // A
        x.store(r1, Relaxed); // B
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Relaxed); // C
        y.store(42, Relaxed); // D
        r2
    });
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    // r1 == r2 == 42
    assert_eq!(r1, 42);
    assert_eq!(r2, 42);
}

#[test]
fn seq_cst() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || x.store(true, SeqCst));
    let _ty = spawn(move || y.store(true, SeqCst));
    let t1 = spawn(move || {
        while !x.load(SeqCst) {}
        if y.load(SeqCst) {
            z.fetch_add(1, Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(SeqCst) {}
        if x.load(SeqCst) {
            z.fetch_add(1, Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(SeqCst);
}
