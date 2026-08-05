//! Using variable reference

fn main() {
    println!("\n");

    let my_number = &mut 120;
    *my_number += 100;

    println!("value of my_number is: {}", my_number);

    println!("\nThe End ...\n");
}
