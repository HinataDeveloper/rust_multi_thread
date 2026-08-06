//! I am learning how to using Mutex.

use std::sync::{Mutex};

fn main() {
    println!("\n");

    let number_list: Mutex<Vec<u32>> = Mutex::new(vec![1, 2, 3]);

    let guard_result = number_list.lock();
    let mut guard = match guard_result {
        Ok(guard) => guard,
        Err(poisond) => {
            eprintln!("Error: {}", poisond);
            poisond.into_inner()
        }
    };

    guard.push(4);
    for item in guard.iter() {
        print!("{} ", item);
    }

    println!("\nThe End ...\n");
}
