use box_t::heap::List::{Cons, Nil};

fn main() {
    let value = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Hello, world! {:?}", value);
}
