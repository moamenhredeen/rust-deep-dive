//! One way to implement a queue is to use a linked list; however, that requires a lot of dynamic memory manipulation to add/remove individual items.
//! A more low-level approach is to use a circular buffer: the compromise is that the capacity of the queue is then "fixed". For a background on circular buffers,
//! you can consult https://en.wikipedia.org/wiki/Circular_buffer
//! A partial implementation is provided below; please finish it and add some more methods; please remember to run 'cargo fmt' and 'cargo clippy' after
//! every step to get feedback from the rust compiler!
//! 1) implement read()
//! 2) the queue now has a fixed size; change the definition so that the data member becomes a Box<[u8]>; you can use the provided function 'make_box' to make
//!     boxed slices of arbitrary sizes. Make changes to your method definitions as needed (the definition of 'write' should not need changes!)
//! 3) change the method 'new()' into 'new(size: usize)' that initializes a ring buffer of the given size (instead of a fixed size of 16); use the 'make_box' function.
//! 4) in a queue that has size N, how many elements can be stored at one time? (test your answer experimentally)
//! 5) EXTRA EXERCISES:
//!   - add a method "has_room" so that "queue.has_room()" is true if and only if writing to the queue will succeed
//!   - add a method "peek" so that "queue.peek()" returns the same thing as "queue.read()", but leaves the element in the queue

struct RingBuffer {
    data: Box<[u8]>,
    start: usize,
    end: usize,
}

/// # ringbuffer impl
impl RingBuffer {
    fn new(size: usize) -> RingBuffer {
        RingBuffer {
            data: make_box(size),
            start: 0,
            end: 0,
        }
    }

    /// This function tries to read a value from the queue and returns Some(value) if this succeeds,
    /// it returns None if the queue was empty
    fn read(&mut self) -> Option<u8> {
        // if the ring buffer was initialized with capacity of zero
        // we can not read from it
        if self.data.len() == 0 {
            return None;
        }

        if self.start == self.end {
            None
        }else {
            let pos = self.start;
            self.start = (self.start + 1) % self.data.len() ;
            Some(self.data[pos])
        }
    }

    /// This function tries to put `value` on the queue; and returns true if this succeeds
    /// It returns false is writing to the queue failed (which can happen if there is not enough room)
    fn write(&mut self, value: u8) -> bool {
        // if the ring buffer was initialized with capacity of zero
        // we can not write to it
        if self.data.len() == 0 {
            return false;
        }

        self.data[self.end] = value;
        let pos = (self.end + 1) % self.data.len();
        if pos == self.start {
            false
        } else {
            self.end = pos;
            true
        }
    }

    fn has_room(&self) -> bool {
        (self.start + 1) % self.data.len() != self.end
    }

    fn peek(&self) -> Option<u8> {
        if self.start == self.end {
            None
        }else {
            Some(self.data[self.start])
        }

    }
}

/// This function creates an "owned slice" a user-selectable size by allocating it as a vector (filled with zeores) using vec![], and then turning it
/// into a Box<[u8]> using the into_boxed_slice() method, see https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice
fn make_box(req_size: usize) -> Box<[u8]> {
    vec![0; req_size].into_boxed_slice()
}

/// This is a fun extra bit: by defining an "iterator", a ring buffer we defined ourselves can be used in for loops! (We will explain this feature in a later module!)

impl Iterator for RingBuffer {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.read()
    }
}

fn main() {
    let mut ring = RingBuffer::new(10);
    ring.write(1);
    ring.write(2);
    ring.write(3);
    if ring.has_room() {
        ring.write(4);
    }
    ring.peek();
    for i in ring {
        println!("ring: {}", i)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_from_zero_capacity() {
        let mut ring = RingBuffer::new(0);
        assert_eq!(ring.read(), None);
    }


    #[test]
    fn test_write_to_zero_capacity() {
        let mut ring = RingBuffer::new(0);
        assert!(!ring.write(10));
    }

    #[test]
    fn test_read_from_empty_ring_buffer(){
        let mut ring = RingBuffer::new(10);
        assert_eq!(ring.read(), None);
    }

    #[test]
    fn test_read_from_non_empty_ring_buffer(){
        let mut ring = RingBuffer::new(10);
        assert!(ring.write(1));
        assert_eq!(ring.read(), Some(1));
    }

    #[test]
    fn test_write_to_full_ring_buffer(){
        let mut ring = RingBuffer::new(2);
        assert!(ring.write(1));
        assert!(!ring.write(2));
    }
}
