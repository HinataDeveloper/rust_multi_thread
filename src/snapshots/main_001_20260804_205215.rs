//! I am learning how to use Mutex.

use std::sync::Mutex;

fn main() {
    println!("\n");

    let counter = Mutex::new(0);

    {
        let mut value = counter.lock().unwrap();
        *value += 1;
    }

    println!("value is: {}", counter.lock().unwrap());

    println!("\n The End ...\n");
}
