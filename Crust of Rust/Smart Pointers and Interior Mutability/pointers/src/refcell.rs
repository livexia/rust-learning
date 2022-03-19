use crate::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Copy)]
enum SharedState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<SharedState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(SharedState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            SharedState::Unshared => {
                self.state.set(SharedState::Shared(1));
                Some(Ref { refcell: self })
            }
            SharedState::Shared(n) => {
                self.state.set(SharedState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            SharedState::Exclusive => panic!("already borrowed: BorrowMutError"),
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let SharedState::Unshared = self.state.get() {
            self.state.set(SharedState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            SharedState::Unshared | SharedState::Exclusive => unreachable!(),
            SharedState::Shared(1) => self.refcell.state.set(SharedState::Unshared),
            SharedState::Shared(n) => self.refcell.state.set(SharedState::Shared(n - 1)),
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            SharedState::Unshared | SharedState::Shared(_) => unreachable!(),
            SharedState::Exclusive => self.refcell.state.set(SharedState::Unshared),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panicked() {
        // use std::cell::RefCell;
        let c = RefCell::new(42);

        {
            let m = c.borrow();
            let b = c.borrow_mut();
            assert!(b.is_none());
        }
        let b = c.borrow_mut();
        assert!(b.is_some());
        // thread 'refcell::tests::it_works' panicked at 'already borrowed: BorrowMutError'
    }
}
