//! while loop take one minute time and then program ends.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    println!("\n");

    let now = Instant::now();
    let loop_duration = Duration::from_secs(60);

    while now.elapsed() < loop_duration {
        println!("{:?}", Instant::now());
        thread::sleep(Duration::from_secs(1));
    }

    println!("\nThe End ...\n");
}
