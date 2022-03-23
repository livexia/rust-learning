use std::ops::Deref;
use std::ops::DerefMut;

pub struct Boks<T> {
    p: *mut T,
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Self {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.p }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p }
    }
}

fn main() {
    let v = 42;
    let b = Boks::ny(v);
    println!("{}", &*b);
}
