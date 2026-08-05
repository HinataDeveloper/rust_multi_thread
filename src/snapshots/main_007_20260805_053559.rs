//! I am learning how to using Mutex.

use std::sync::Mutex;

fn main() {
    println!("\n");

    let my_number = 120;

    // my_number is a primitive type. then copies into the 
    // raphael mutex.
    let raphael = Mutex::new(my_number);

    {
        let mut samuel = raphael.lock().unwrap();
        *samuel += 100;
    }

    println!("value of my_number is: {}", my_number);
    println!("raphael keep this value: {}", *raphael.lock().unwrap());

    println!("\nThe End ...\n");
}
