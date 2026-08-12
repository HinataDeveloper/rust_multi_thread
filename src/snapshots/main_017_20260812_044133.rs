//! Sharing data between threads.
//! How to use Box?
//! Box passing to the function.

fn main() {
    println!("\n");

    let raphael = Box::new(500);
    let samuel = Box::new(300);

    let resultant_one = add_two_box(raphael, samuel);

    let robert = Box::new(900);
    let albert = Box::new(1100);

    let resultant_two = add_two_box(robert, albert);

    let total = *add_two_box(resultant_one, resultant_two);

    println!("value of total is: {}", total);

    println!("\nThe End ...\n");
}

fn add_two_box(x: Box<i32>, y: Box<i32>) -> Box<i32> {
    Box::new(*x + *y)
}
