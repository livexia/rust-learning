use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = 42;
        let c = Cell::new(v);
        assert_eq!(c.get(), 42);
        c.set(43);
        assert_eq!(c.get(), 43);
    }

    #[test]
    fn std_cell_doc_code() {
        struct SomeStruct {
            #[allow(dead_code)]
            regular_field: u8,
            special_field: Cell<u8>,
        }

        let my_struct = SomeStruct {
            regular_field: 0,
            special_field: Cell::new(42),
        };

        let new_value = 255;

        my_struct.special_field.set(new_value);
        assert_eq!(my_struct.special_field.get(), new_value);
    }
}
