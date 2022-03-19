use crate::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Self {
            // SAFETY: Box doesn't give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc drop
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let n = inner.refcount.get();
        inner.refcount.set(n + 1);
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // not needed see: https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8?permalink_comment_id=3805900#gistcomment-3805900
            // should replace drop(inner); with let inner = ();
            drop(inner);
            // SAFETY: we are the _only_ Rc left, and we are being dropped.
            // therefore, after us, there will be no Rc, and no reference to T/
            let _ = unsafe { self.inner.as_ref() };
        } else {
            // there are other Rc, so don't drop the box.
            inner.refcount.set(c - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let five = Rc::new(5);

        {
            let f_c = five.clone();
            assert_eq!(2, unsafe { f_c.inner.as_ref().refcount.get() });
        }
        assert_eq!(1, unsafe { five.inner.as_ref().refcount.get() });

        assert_eq!(*five, 5);
    }

    #[test]
    fn rc_with_cell() {
        let five = Rc::new(Cell::new(5));

        {
            assert_eq!(five.get(), 5);
            let f_c = five.clone();
            f_c.set(10);
            // assert_eq!(2, unsafe { f_c.inner.as_ref().refcount.get() });
        }
        // assert_eq!(1, unsafe { five.inner.as_ref().refcount.get() });

        assert_eq!(five.get(), 10);
    }
}
