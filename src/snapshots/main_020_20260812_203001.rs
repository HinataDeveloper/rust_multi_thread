//! Sharing data between threads project.
//! How to use Box?
//! Without dependency.
//! Rust Version nightly 1.99.0

fn main() {
    println!("\n");

    let mut raphael = 300;
    let mut samuel = Box::new(&mut raphael);

    **samuel += 500;
    println!("value of samuel: {}", **samuel);
    println!("value of raphael: {}", raphael);
    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    let mut albert = 900;
    samuel = Box::new(&mut albert);

    **samuel += 1300;

    println!("value of samuel: {}", **samuel);
    println!("value of albert: {}", albert);

    println!("\nThe End ...\n");
}
