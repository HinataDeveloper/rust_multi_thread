//! Sharing data between threads project.
//! How to use Box?
//! Without dependency.
//! Rust Version nightly 1.99.0

fn main() {
    println!("\n");

    let mut raphael = 300;
    let mut samuel = Box::new(&raphael);

    // encounter with compile time error. 
    // because raphael variable was borrowed and not consume yat.
    raphael += 500;
    
    // *samuel += 900;
    
    println!("value of samuel: {}", *samuel);
    println!("value of raphael: {}", raphael);

    println!("\nThe End ...\n");
}
