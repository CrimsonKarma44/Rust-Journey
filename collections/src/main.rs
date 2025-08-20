use std::io;
use collection::{array::array_looper, vector::{looper, storing_multiple_datatypes}, hash_maps::word_count};

pub mod collection;

fn main() {
    let mut new_vector_array = vec![1,2,3,4,5];
    test(&new_vector_array);
    // loop;
    array_looper();
    looper(&mut new_vector_array);
    storing_multiple_datatypes();
    word_count();
}

fn test(val: &Vec<i32>){
    println!("test funtion...");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("wrong");
    let index: usize = value.trim().parse().expect("invalid conversion");

    let getter = val.get(index);
    match getter {
        Some(val) => println!("{}", val),
        None => println!("There is nothing at this location..."),
    }
}