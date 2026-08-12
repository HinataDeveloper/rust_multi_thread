//! ...
//! ...
//! With dependency to rand crate.
//! Rust version nightly 1.99.0

use rand::{RngExt, distr::uniform::SampleRange};
use std::{cmp::Ordering, sync::mpsc::channel, thread};

fn main() {
    println!("\n");

    let (tx1, rx1) = channel::<GameCommand>();
    let (tx2, rx2) = channel::<GameCommand>();

    let gamer_one = thread::spawn(move || {
        let mut domain = Domain::build_default();
        let selected_number = rand::rng().random_range(domain.get_range());

        
    });

    let gamer_two = thread::spawn(move || {});

    println!("\nThe End ...\n");
}

//--------------------------------------------

struct Domain {
    min: u32,
    max: u32,
}

impl Domain {
    fn build_default() -> Domain {
        Self {
            min: 12345,
            max: 98765,
        }
    }

    fn build_from(min: u32, max: u32) -> Domain {
        if min > max {
            panic!("min must be less than max ...");
        }

        Self { min, max }
    }

    fn set_min(&mut self, min: u32) {
        self.min = min;
    }

    fn set_max(&mut self, max: u32) {
        self.max = max
    }

    fn reset(&mut self, min: u32, max: u32) {
        if min > max {
            panic!("min must be less than max ...");
        }

        self.min = min;
        self.max = max;
    }

    fn get_min(&self) -> u32 {
        self.min
    }

    fn get_max(&self) -> u32 {
        self.max
    }

    fn get_range(&self) -> impl SampleRange<u32> {
         self.min..=self.max
    }


}



enum GameCommand {
    Ready,
    Equal,
    Greater,
    Less,
    Number(u32, Domain),
}
