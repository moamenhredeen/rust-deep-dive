use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use crate::interior_mutability::cell::Cell;

#[allow(unused)]
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    references: Cell<isize>,
}

#[allow(unused)]
impl<'a, T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            references: Cell::new(0),
        }
    }

    pub fn borrow(&'a self) -> Option<Ref<'a, T>> {
        let ref_count = self.references.get();
        if ref_count >= 0 {
            self.references.set(ref_count + 1);
            Some(Ref { refcell: self })
        } else {
            None
        }
    }

    pub fn borrow_mut(&'a self) -> Option<RefMut<'a, T>> {
        if self.references.get() == 0 {
            self.references.set(-1);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        let ref_count = self.refcell.references.get();
        if ref_count <= 0 {
            unreachable!();
        } else {
            self.refcell.references.set(ref_count - 1);
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        let ref_count = self.refcell.references.get();
        if ref_count != -1 {
            unreachable!();
        } else {
            self.refcell.references.set(0);
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
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let rc = RefCell::new(10);
        let r = rc.borrow().unwrap();
        assert_eq!(10, *r);
    }

    #[test]
    fn test_case_2() {
        let rc = RefCell::new(10);
        {
            let mut r = rc.borrow_mut().unwrap();
            *r = 20;
        }
        let r = rc.borrow().unwrap();
        assert_eq!(20, *r);
    }
}
