// Date: Thu Aug 15 2026

// Project: Sharing data between threads
// Goal: Share a immutable variable between two threads
// Dependency: Without dependency

// Hardware Mode: Lenovo IdeaPad Z500
// Processor: Intel® Core™ i7-3612QM × 8

// OS: Ubuntu 26.04 LTS x86_64 GNU/Linux
// Kernel Version: 7.0.14-070014-generic
// Firmware Version: 71CN51WW(V1.21)

// Graphics: Intel® HD Graphics 4000 (IVB GT2)
// GNOME Version: 50
// Windowing System: Wayland

// rustc 1.99.0-nightly (d453bdd8f 2026-08-14)
// cargo 1.99.0-nightly (eb98b54bc 2026-08-11)
// rustup 1.29.0 (28d1352db 2026-03-05)

use std::thread::{self, JoinHandle};

fn main() {
    println!("\n");

    let raphael = String::from("I am Raphael Gray");

    let thread_one: JoinHandle<()> = thread::spawn(move || {
        println!("How are you? {}", raphael);
    });

    // encounter with compile time error.
    // because raphale moved into previous thread.
    let thread_two: JoinHandle<()> = thread::spawn(move || {
        println!("How am I? {}", raphael);
    });

    thread_one.join().unwrap();
    thread_two.join().unwrap();

    println!("\nThe End ...\n");
}
