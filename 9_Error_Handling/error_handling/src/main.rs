use error_handling::*;

pub mod error_handling;
fn main() {
    println!("Hello, world!");
    // unrecoverable_error();
    recoverable_error();
    let value:Option<i32> = None;
    match test(value) {
        Ok(t) => println!("{}", t),
        Err(n) => panic!("{}", n),
    }
}

