// Date: Thu Aug 13 2026

// Project: Sharing data between threads
// Goal: How to use Box
// Dependency: Without dependency

// Hardware Mode: Lenovo IdeaPad Z500
// Processor: Intel® Core™ i7-3612QM × 8

// OS: Ubuntu 26.04 LTS x86_64 GNU/Linux
// Kernel Version: 7.0.14-070014-generic
// Firmware Version: 71CN51WW(V1.21)

// Graphics: Intel® HD Graphics 4000 (IVB GT2)
// GNOME Version: 50
// Windowing System: Wayland

// rustc 1.99.0-nightly (3d6c19bb9 2026-08-11)
// cargo 1.99.0-nightly (b07e5a086 2026-08-07)
// rustup 1.29.0 (28d1352db 2026-03-05)

fn main() {
    println!("\n");

    let message = &mut String::from("I am a beginner Rustacean ...");
    let message_box = Box::new(message);

    println!("value of message is: {}", message_box);

    // encounter with compile time error
    // because message is move into message_box
    println!("value of message is: {}", message);

    println!("\nThe End ...\n");
}
