//! Guess Game
//! Thread one, choose a number between 0 and 100.
//! Thread two, try to find it.

use std::{cmp::Ordering, sync::mpsc::channel};

use rand::RngExt;

fn main() {
    println!("\n");

    let (tx1, rx1) = channel::<Commands>();
    let (tx2, rx2) = channel::<Commands>();

    let chosen_number: u32 = rand::rng().random_range(0..101);

    println!("\nThe End ...\n");
}

enum Commands {
    Ready,
    GuessNumber(u32),
    Response(Ordering),
}

struct LastTwoNumber {
    one: Option<u32>,
    two: Option<u32>,
}

impl LastTwoNumber {
    fn is_none(&self) -> bool {
        self.one == None && self.two == None
    }

    fn only_two_is_none(&self) -> bool {
        self.one != None && self.two == None
    }

    fn set_one(&mut self, one: Option<u32>) {
        self.one = one;
    }

    fn set_two(&mut self, two: Option<u32>) {
        self.two = two;
    }

    fn get_one(&self) -> Option<u32> {
        self.one
    }

    fn get_two(&self) -> Option<u32> {
        self.two
    }


}

const MSG_GT: &str = "Your number is greater than guessed number ...";
const MSG_LT: &str = "Your number is less than guessed number ...";
const MSG_EQ: &str = "Your number is equals guessed number ...";
const MSG_WIN: &str = "Good Job You Win ...";

fn guess_new_number(last_response: Ordering, last_numbers: LastTwoNumber) -> u32 {
    
    let last_number: u32;
    let before_last_number: u32;
    
    if last_numbers.is_none() {
        return 50;
    }
    
    if  last_numbers.only_two_is_none() {
        last_number = last_numbers.get_one().unwrap();
        
    } else {
        last_number = last_numbers.get_two().unwrap();
    }
    
    
    
     if let Some(num) = last_numbers.get_two() {
        match last_response {
            Ordering::Equal => return num,
            Ordering::Greater => return ((num + 1) + 100) / 2,
            Ordering::Less => {
                unimplemented!();
            }
        }
    }

    0
}
