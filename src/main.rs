//! Select a random number and then try for finding it
//! by binary search pattern.
//! With dependency to rand crate.
//! Rust version nightly 1.99.0

use rand::RngExt;
use std::cmp::Ordering;

fn main() {
    println!("\n");

    let mut min: u32 = 12345;
    let mut max: u32 = 98765;
    let mut loop_count: u32 = 0;
    let mut guess_number: u32;
    let my_selected_number: u32 = rand::rng().random_range(min..=max);

    'guess: loop {
        loop_count += 1;

        guess_number = (min + max) / 2;

        match &guess_number.cmp(&my_selected_number) {
            Ordering::Equal => {
                println!("I found the selected number. it is: {}", guess_number);
                println!("loop count: {}", loop_count);
                break 'guess;
            }
            Ordering::Greater => {
                max = guess_number - 1;
            }
            Ordering::Less => {
                min = guess_number + 1;
            }
        } // end match
    } // end loop

    println!("\nThe End ...\n");
}
