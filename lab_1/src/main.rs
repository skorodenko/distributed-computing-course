use std::{thread};
use std::sync::{Arc, Mutex};
use thread_priority::*;


fn increment(refr: &Arc<Mutex<i32>>) {
    let arc = Arc::clone(&refr);
    let mut num = arc.lock().unwrap();
    *num += 1;
}

fn decrement(refr: &Arc<Mutex<i32>>) {
    let arc = Arc::clone(&refr);
    let mut num = arc.lock().unwrap();
    *num -= 1;
}

fn main() {
    let shared: Arc<Mutex<i32>>= Arc::new(Mutex::new(0));
    
    let increment_shared = shared.clone();
    let handle_increment = thread::spawn(move || {
        //assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        loop {
            increment(&increment_shared);
            let num = increment_shared.lock().unwrap();
            println!("Incerment thread :: {num}");
        }
    });
    
    let decrement_shared = shared.clone();
    let handle_decrement = thread::spawn(move || {
        //assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        loop {
            decrement(&decrement_shared);
            let num = decrement_shared.lock().unwrap();
            println!("Decrement thread :: {num}");
        }
    });

    handle_increment.join().unwrap();
    handle_decrement.join().unwrap();
}
