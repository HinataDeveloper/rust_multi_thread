//! I am learning how to using Mutex.

use std::{sync::{Arc, Mutex}, thread::JoinHandle};

fn main() {
    println!("\n");

    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(0_u64));
    let thread_pool: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..10 {
        
    }


    println!("\nThe End ...\n");
}
