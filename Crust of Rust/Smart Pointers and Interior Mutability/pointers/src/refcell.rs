use std::cell::UnsafeCell;

pub struct RefCell<T> {
    value: UnsafeCell<T>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn borrow(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panicked() {
        use std::cell::RefCell;
        let c = RefCell::new(42);

        let m = c.borrow();
        let b = c.borrow_mut();
        // thread 'refcell::tests::it_works' panicked at 'already borrowed: BorrowMutError'
    }
}
