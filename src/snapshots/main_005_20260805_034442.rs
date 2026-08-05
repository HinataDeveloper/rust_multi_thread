//! I ma learning Mutex

use std::sync::Mutex;

fn main() {
    println!("\n");

    let mut my_number = 120;
    let raphael = Mutex::new(&mut my_number);

    {
        let mut samuel = raphael.lock().unwrap();
        **samuel += 100;
    }

    println!("value of samuel is: {}", my_number);

    println!("\nThe End ...\n");
}
