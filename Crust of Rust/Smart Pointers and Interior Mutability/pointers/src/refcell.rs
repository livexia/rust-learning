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

// implied by UnsafeCell
// RefCell is !Sync

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(SharedState::Unshared),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        match self.state.get() {
            SharedState::Unshared => {
                self.state.set(SharedState::Shared(1));
                Ref { refcell: self }
            }
            SharedState::Shared(n) => {
                self.state.set(SharedState::Shared(n + 1));
                Ref { refcell: self }
            }
            SharedState::Exclusive => panic!("already mutably borrowed: BorrowError"),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        if let SharedState::Unshared = self.state.get() {
            self.state.set(SharedState::Exclusive);
            // SAFETY: no other references have been given out since state would be
            // Shared or Exclusive
            RefMut { refcell: self }
        } else {
            panic!("already borrowed: BorrowMutError")
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY
        // a Ref is only created id no exclusive references have been given out.
        // once it is given out, state is set to Shared, so no exclusive referneces are given out
        // so dereferening into a shared reference is fine
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
        // SAFETY
        // see safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY
        // a RefMut is only created if no other references have been given out.
        // once it is given out, state is set to Exclusive, so no further referneces are given out
        // so we have an exclusive lease on the inner value, so mutably dereferening is fine
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panicked() {
        // use std::cell::RefCell;
        let c = RefCell::new(42);

        let m = c.borrow();
        let b = c.borrow_mut();
        // thread 'refcell::tests::it_works' panicked at 'already borrowed: BorrowMutError'
    }
}
