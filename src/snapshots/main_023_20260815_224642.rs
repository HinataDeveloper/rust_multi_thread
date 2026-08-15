// Date: Thu Aug 15 2026

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

// rustc 1.99.0-nightly (d453bdd8f 2026-08-14
// cargo 1.99.0-nightly (eb98b54bc 2026-08-11)
// rustup 1.29.0 (28d1352db 2026-03-05)

fn main() {
    println!("\n");

    let message = String::from("I am a beginner Rustacean ...");

    // message move into messag_box.
    let mut message_box = Box::new(message);

    println!("value of message_box is: {}", message_box);

    message_box.push_str(" I am trying ...");

    println!("value of message_box is: {}", message_box);

    println!("\nThe End ...\n");
}
