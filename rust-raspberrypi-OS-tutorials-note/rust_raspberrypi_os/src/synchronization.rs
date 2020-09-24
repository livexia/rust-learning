use core::cell::UnsafeCell;

pub mod interface {
    pub trait Mutex {
        type Data;

        fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}


pub struct NullLock<T: ?Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized> Sync for NullLock<T> {}

impl<T> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}


impl<T> interface::Mutex for &NullLock<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}