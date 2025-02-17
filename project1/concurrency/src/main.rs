//use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut i = counter.lock().unwrap();
            *i += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {}", counter.lock().unwrap());
}