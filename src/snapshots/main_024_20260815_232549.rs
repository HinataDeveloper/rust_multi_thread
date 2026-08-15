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

// rustc 1.99.0-nightly (d453bdd8f 2026-08-14)
// cargo 1.99.0-nightly (eb98b54bc 2026-08-11)
// rustup 1.29.0 (28d1352db 2026-03-05)

use human::Person;

const SEPARATOR: &str = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";

fn main() {
    println!("\n");

    let raphael: Person = Person::build(2003, "Raphael", "Gray", 64);
    let mut raphael_box = Box::new(raphael);

    println!("before ->>> value of raphael_box: {}", *raphael_box);

    raphael_box.set_person_id(5003);
    raphael_box.set_first_name("Samuel");
    raphael_box.set_last_name("Brown");
    raphael_box.set_age(69);
    println!("{}", SEPARATOR);

    println!("after ->>> value of raphael_box: {}", raphael_box);

    raphael_box.set_person_id(90239);
    raphael_box.set_first_name("Albert");
    raphael_box.set_last_name("Enstain");
    raphael_box.set_age(71);
    println!("{}", SEPARATOR);

    println!("person ID: {}", raphael_box.get_person_id());
    println!("first name: {}", raphael_box.get_first_name());
    println!("last name: {}", raphael_box.get_last_name());
    println!("age: {}", raphael_box.get_age());
    println!("{}", SEPARATOR);

    println!("\nThe End ...\n");
}

mod human {
    use std::fmt::Display;

    pub struct Person {
        person_id: u64,
        first_name: String,
        last_name: String,
        age: u32,
    }

    impl Person {
        pub fn build(
            person_id: u64,
            first_name: impl Into<String>,
            last_name: impl Into<String>,
            age: u32,
        ) -> Person {
            Self {
                person_id,
                first_name: first_name.into(),
                last_name: last_name.into(),
                age,
            }
        }

        pub fn set_person_id(&mut self, person_id: u64) {
            self.person_id = person_id;
        }

        pub fn set_first_name(&mut self, first_name: impl Into<String>) {
            self.first_name = first_name.into();
        }

        pub fn set_last_name(&mut self, last_name: impl Into<String>) {
            self.last_name = last_name.into();
        }

        pub fn set_age(&mut self, age: u32) {
            self.age = age;
        }

        pub fn get_person_id(&self) -> u64 {
            self.person_id
        }

        pub fn get_first_name(&self) -> &str {
            &self.first_name
        }

        pub fn get_last_name(&self) -> &str {
            &self.last_name
        }

        pub fn get_age(&self) -> u32 {
            self.age
        }
    }

    impl Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "[person_id:{}, first_name:{}, last_name:{}, age:{}]",
                self.person_id, self.first_name, self.last_name, self.age
            )
        }
    }
}
