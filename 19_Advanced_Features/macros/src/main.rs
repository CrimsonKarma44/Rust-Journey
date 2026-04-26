use macros::myVec;
use hello_macro_derive::HelloMacro;

struct Pancakes;

// inefficient cos of repeated implementation
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, macro! My name is Pancakes!");
    }
}

fn main() {
    let some_vector: Vec<i32> = myVec![];
    let some_vector: Vec<i32> = vec![];
    
    println!("Welcome to Meta Programming! {:?}", some_vector);
    Pancakes::hello_macro();
}


