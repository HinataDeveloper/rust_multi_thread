//! I am learning how to use Mutex.

fn main() {
    println!("\n");

    let my_number = &mut 120;
    *my_number += 100;
    println!("value of my number is: {}", *my_number);

    println!("\nThe End ...\n");
}
