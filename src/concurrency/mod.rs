//! Sync, Send

use std::{sync::mpsc, thread, time::Duration};

#[allow(unused)]
pub fn multi_threaded_printing() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let producer_1 = thread::spawn(move || {
        for i in 0..3 {
            let message = format!("(producer: 1, message: {})", i);
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(rand::random_range(0..10)));
        }
    });

    let producer_2 = thread::spawn(move || {
        for i in 0..3 {
            let message = format!("(producer: 2, message: {})", i);
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(rand::random_range(0..10)));
        }
    });

    for m in rx {
        println!("{}", m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        rc::Rc,
        sync::{Arc, Mutex},
        thread::JoinHandle,
    };

    // #[test]
    // fn test_multi_threaded_printing() {
    //     multi_threaded_printing();
    // }

    #[test]
    fn mutex_demo_test() {
        let mutex = Arc::new(Mutex::new(0));
        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        for thread_number in 0..10 {
            let m = Arc::clone(&mutex);
            threads.push(thread::spawn(move || {
                for _ in 0..100 {
                    let mut mutex_v = m.lock().unwrap();
                    *mutex_v += 1;
                    println!("{} working", thread_number);
                }
            }));
        }
        for t in threads {
            t.join().unwrap();
        }

        println!("i = {}", *mutex.lock().unwrap());
    }
}
