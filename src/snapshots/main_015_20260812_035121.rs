//! Sharing data between threads.
//! how to use Box?

fn main() {
    println!("\n");

    let mut raphael = Box::new(300);
    let mut samuel = Box::new(500);

    println!("raphael: {}, samuel: {}", raphael, samuel);

    *samuel += *raphael;

    println!("raphael: {}, samuel: {}", raphael, samuel);

    *raphael += *samuel;

    println!("raphael: {}, samuel: {}", raphael, samuel);

    println!("\nThe End ...\n");
}
