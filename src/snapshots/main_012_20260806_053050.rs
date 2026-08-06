//! I am learning how to using Mutex.

use std::sync::Mutex;

fn main() {
    println!("\n");

    let mut number_list: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let append_number_list: Vec<u32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

    println!("Number list before append: ");
    show_arr_num_u32(&number_list);

    let mutex_number_list: Mutex<&mut Vec<u32>> = Mutex::new(&mut number_list);
    append_arr_num_u32(&mutex_number_list, &append_number_list);

    println!("Number list after append: ");
    show_arr_num_u32(&number_list);

    println!("\nThe End ...\n");
}

fn show_arr_num_u32(data: &[u32]) {
    for item in data.iter() {
        print!("{} ", item);
    }
    println!();
}

fn append_arr_num_u32(source_data: &Mutex<&mut Vec<u32>>, input_data: &[u32]) {
    let guard_result = source_data.lock();
    let mut guard = match guard_result {
        Ok(guard) => guard,
        Err(poison_err) => {
            eprintln!("Error: {}", poison_err);
            poison_err.into_inner()
        }
    };

    for item in input_data.iter() {
        guard.push(*item);
    }
}
