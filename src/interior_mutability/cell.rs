use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

#[allow(unused)]
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let c = Cell::new(10);
        assert_eq!(10, c.get());
        c.set(20);
        assert_eq!(20, c.get());
    }
}
