//! I am learning how to using Mutex.

use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

fn main() {
    println!("\n");

    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(0_u64));
    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..10 {
        let arc_counter = Arc::clone(&counter);

        let handel = thread::spawn(move || {
            for _ in 0..1000 {
                let mut value: MutexGuard<u64> = arc_counter.lock().unwrap();
                *value += 1;
            }
        });

        thread_pool.push(handel);
    }

    for trd in thread_pool {
        trd.join().unwrap();
    }

    let resultant: MutexGuard<u64> = counter.lock().unwrap();
    println!("value of resultant is: {}", *resultant);
    drop(resultant);

    println!("\nThe End ...\n");
}
