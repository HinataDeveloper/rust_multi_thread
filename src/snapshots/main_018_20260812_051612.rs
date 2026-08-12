//! Sharing data between threads.
//! How to use Box?

fn main() {
    println!("\n");

    let mut raphael = 300;
    let mut samuel = Box::new(raphael);

    raphael += 900;
    *samuel += 500;

    println!("value of raphael: {}", raphael);
    println!("value of samuel: {}", samuel);

    println!("\nThe End ...\n");
}
