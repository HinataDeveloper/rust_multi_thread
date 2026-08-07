//! Using channel to send data between two thread.

use std::{sync::mpsc::channel, thread, time::Duration};

fn main() {
    println!("\n");
    let (tx, tr) = channel::<i32>();

    
    let thread_one = thread::spawn(move || {
        for item in 0..100 {
            tx.send(item).unwrap();
            // thread::sleep(Duration::from_micros(500));
        }
    });

    let thread_two = thread::spawn(move || {
        for _ in 0..100 {
            let rec = tr.recv().unwrap();
            println!("I got this: {}", rec);
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread_one.join().unwrap();
    thread_two.join().unwrap();

    println!("\nThe End ...\n");
}
