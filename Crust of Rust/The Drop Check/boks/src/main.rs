use std::ops::Deref;
use std::ops::DerefMut;

pub struct Boks<T> {
    p: *mut T,
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Self {
            // use box to create data on heap, always return nonnull pointer
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: this is fine because the pointer came from a Box,
        // so it is safe to convert back to a box to drop it.
        let _ = unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: pointer came from a Box<T>,
        // so it is fine to return a &T from the pointers
        unsafe { &*self.p }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: pointer came from a Box<T>,
        // so it is fine to return a &mut T from the pointer
        unsafe { &mut *self.p }
    }
}

fn main() {
    let v = 42;
    let b = Boks::ny(v);
    println!("b: {}", &*b);
}
