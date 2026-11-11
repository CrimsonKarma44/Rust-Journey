use modules::*;

use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
    eat_at_restaurant();
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}