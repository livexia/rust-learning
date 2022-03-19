pub struct RefCell<T> {
    value: T,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn borrow(&self) -> &T {
        &self.value
    }

    pub fn borrow_mut(&self) -> &mut T {
        &mut self.value
    }
}
