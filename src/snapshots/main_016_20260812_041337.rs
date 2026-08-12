//! sharing data between threads.
//! how to use Box?

fn main() {
    println!("\n");

    let kevin = 49;
    let mut raphael = Box::new(kevin);

    *raphael = 500;

    println!("value of kevin: {}", kevin);
    println!("value of raphael: {}", *raphael);

    println!("\nThe End ...\n");
}
