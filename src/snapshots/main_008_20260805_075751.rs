//! I am learning how to using Mutex.

use std::sync::Mutex;

fn main() {
    println!("\n");

    let raphael: Mutex<Person> = Mutex::new(Person {
        person_id: 2003,
        first_name: String::from("Raphael"),
        last_name: String::from("Gray"),
        age: 64,
    });

    println!("before ->>> raphael: {:?}\n", raphael);

    {
        let mut raph = raphael.lock().unwrap();
        raph.person_id = 50001;
        raph.first_name = String::from("Samuel");
        raph.last_name = String::from("Brown");
        raph.age = 79;
    } 
    // The guard is dropped automatically when the critical section is enclosed in a block.
    // 

    println!("after ->>> raphael: {:?}", raphael);

    println!("\nThe End ...\n");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    person_id: u64,
    first_name: String,
    last_name: String,
    age: u32,
}
