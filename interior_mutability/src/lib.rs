//! Smart pointers and interior mutability
//! - Cell

use std::cell::UnsafeCell;

/// reimplementation of Cell to be able to understand it
struct MyCell<T: Copy> {
    // the first obvious attempt would not work, because you can mutate
    // a variable behind immutable reference
    //value: T,

    // the way cell enable us of mutating a variable behind a shared reference
    // is by using UnsafeCell, which uses unsafe and is also unsafe to use,
    // to provide us with interior mutability in rust
    value: UnsafeCell<T>,
}
impl<T: Copy> MyCell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> T {
        // SAFETY: because the value of type copy, no one gets a reference to the value,
        // and therefore it is safe
        unsafe { *self.value.get() }
    }

    fn set(&self, value: T)  {
        // SAFETY: we know, that it is not possible to mutate MyCell value from multiple threads,
        // because MyCell implement !Sync.
        unsafe { *self.value.get() = value }
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, thread};

    use crate::MyCell;

    #[test]
    fn create_cell_and_get_value_from_it() {
        let cell = MyCell::new(10);
        assert_eq!(cell.get(), 10)
    }

    #[test]
    fn create_cell_and_mutate_it() {
        let cell = MyCell::new(10);
        cell.set(20);
        let value = cell.get(); 
        assert_eq!(value, 20)
    }
    
    
    #[test]
    fn create_cell_and_mutate_it_from_multiple_threads() {
        // important note: to be able to mutable reference to a value between multiple threads
        // you have to implement `!Sync` trait
        // but UnsafeCell already implement !Sync `impl<T: ?Sized> !Sync for UnsafeCell<T> {}`
        // our cell implementation inherit this property and therefore also implement `!Sync`
        //
        // note: if UnsafeCell did not implement `!Sync` we had to implement it ourself: 
        // first method (kind of workaround): add a member to the struct that implement `!Sync`
        // second method: implement `!Sync` trait (not sure if it is possible right now) 
        // let cell = Arc::new(MyCell::new(10));
        
        // let a = cell.clone();
        // thread::spawn(|| {
        //     a.set(10);
        // });

        // let b = cell.clone();
        // thread::spawn(|| {
        //     b.set(20);
        // });

        // could not set up multithreaded test 😅
        // I need to understand how `Arc` works to be able to share
        // a reference between threads in a safe way
        // i also have to understand `Sync` and `Send` traits, which pretty important
        // for multi-threading in rust
    }



    #[test]
    fn create_cell_and_and_mutate_it_using_multiple_shared_references() {
        let cell = MyCell::new(10);
        let shared_ref_1 = &cell;
        let shared_ref_2 = &cell;

        shared_ref_1.set(2);
        shared_ref_2.set(3);
        assert_eq!(cell.get(), 3)
    }
}
