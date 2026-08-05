//! I am learning how to using Mutex.

use std::sync::Mutex;

fn main() {
    println!("\n");

    let my_number = Mutex::new(1000);
    add_to_number(&my_number, 500);

    println!("value of my number is: {}", *my_number.lock().unwrap());

    println!("\nThe End ...\n");
}

fn add_to_number(data: &Mutex<i32>, number: i32) {
    let mut raphael = data.lock().unwrap();
    *raphael += number;
}
