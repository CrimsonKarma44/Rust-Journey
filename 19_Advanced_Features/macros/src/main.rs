use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, my_logger, sql};
use macros::myVec;
// use macros::procedural_macros::custom::*;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // Declarative macro
    // let some_vector: Vec<i32> = vec![];
    let value = |some_vector| -> Option<Vec<i32>> {
        let val = some_vector?;
        Some(val)
    };
    println!("Welcome to Meta Programming! {:?}", value(Some(myVec![])));
    
    // custom derive macro
    Pancakes::hello_macro();

    // attribute like macro
    just_some_function();

    // function like macro
    let result = sql!(SELECT * FROM posts WHERE id=1);
    println!("{:?}", result);
}

#[my_logger]
fn just_some_function() {
    println!("just does some function")
}