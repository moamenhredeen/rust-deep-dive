//! reimplement rc from scratch for learning purposes

use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use crate::interior_mutability::cell::Cell;


#[allow(unused)]
struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>
}

#[allow(unused)]
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    // TODO: Drop check
    _marker: PhantomData<RcInner<T>>
}

#[allow(unused)]
impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let rc_data = Box::new(RcInner{
            value,
            ref_count: Cell::new(1)
        });
        Rc {
            inner: NonNull::new(Box::into_raw(rc_data)).unwrap(),
            _marker: PhantomData
        }
    }
}


impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // TODO: incremnent references count
        let inner = unsafe { self.inner.as_ref() } ;
        inner.ref_count.set(inner.ref_count.get() + 1);
        Self { inner: self.inner, _marker: PhantomData }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.inner.as_ref() };
        &inner.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let ref_count = inner.ref_count.get();
        if ref_count == 1 {
            let _ = inner;
            let _ = unsafe { Box::from_raw(self.inner.as_ptr())};
        } else {
            inner.ref_count.set(ref_count - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let rc = Rc::new(String::from("hello world"));
        let rc2 = rc.clone();
        let inner = unsafe { rc2.inner.as_ref() };
        assert_eq!(2,  inner.ref_count.get())
    }


    #[test]
    fn test_deref() {
        let rc = Rc::new(String::from("Hello World"));
        assert_eq!(String::from("Hello World"),  *rc)
    }


    #[test]
    fn test_drop() {
        let rc = Rc::new(String::from("Hello World"));
        assert_eq!(1, unsafe { rc.inner.as_ref().ref_count.get() });

        let ref_1 = rc.clone();
        assert_eq!(2, unsafe { rc.inner.as_ref().ref_count.get() });

        let ref_2 = rc.clone();
        assert_eq!(3, unsafe { rc.inner.as_ref().ref_count.get() });

        drop(ref_2);
        assert_eq!(2, unsafe { rc.inner.as_ref().ref_count.get() });


        drop(ref_1);
        assert_eq!(1, unsafe { rc.inner.as_ref().ref_count.get() });

        drop(rc);
    }
}
