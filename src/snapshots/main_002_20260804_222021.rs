//! I a learning how to use Mutex.

use std::sync::Mutex;

fn main() {
    println!("\n");

    let mut my_number = 120;
    let mutext_number = Mutex::new(&mut my_number);

    {
        let mut resultant = mutext_number.lock().unwrap();
        **resultant += 10;
    }

    println!("value of my number is: {}", my_number);

    println!("\nThe End ...\n");
}
