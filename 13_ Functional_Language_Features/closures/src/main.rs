mod types;
use types::{
    default::default_test,
    borrowing_mutably::test_borrowing_mutably,
    borrowing_immutably::{test_borrowing_immutably, test_array},
    taking_ownership::test_taking_ownership,
};
fn main() {
    default_test();
    println!("------------------");
    test_borrowing_mutably();
    println!("------------------");
    test_borrowing_immutably();
    println!("------------------");
    test_array();
    println!("------------------");
    test_taking_ownership();
    println!("------------------");
}


