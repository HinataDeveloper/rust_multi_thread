//! I am learning how to using Mutex.

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn main() {
    println!("\n");

    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(0_u64));
    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..10 {
        let arc_counter: Arc<Mutex<u64>> = Arc::clone(&counter);

        let handle: JoinHandle<()> = thread::spawn(move || {
            for _ in 0..1000 {
                let mut value = arc_counter.lock().unwrap();
                *value += 1;
            }
        });

        thread_pool.push(handle);
    }

    for trd in thread_pool {
        trd.join().unwrap();
    }

    let resultant = counter.lock().unwrap();
    println!("value is: {}", resultant);

    println!("\nThe End ...\n");
}
