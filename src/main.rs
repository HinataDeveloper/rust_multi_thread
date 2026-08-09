//! I am learning how to using Mutex.

use std::{cmp::Ordering, sync::mpsc::channel};

use rand::RngExt;

fn main() {
    println!("\n");

    let (tx1, rx1) = channel::<Commands>();
    let (tx2, rx2) = channel::<Commands>();

    let chosen_number: u32 = rand::rng().random_range(0..101);

    println!("\nThe End ...\n");
}

enum Commands {
    Ready,
    Guess_Number(u32),
    Response(Ordering),
}

const MSG_GT: &str = "Your number is greater than guessed number ...";
const MSG_LT: &str = "Your number is less than guessed number ...";
const MSG_EQ: &str = "Your number is equals guessed number ...";
const MSG_WIN: &str = "Good Job You Win ...";
