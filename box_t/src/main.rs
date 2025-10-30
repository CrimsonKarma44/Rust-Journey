use box_t::heap::List::{Cons, Nil};
use box_t::smart_pointer::MyBox;
use box_t::drop::CustomerSmartPointer;
use box_t::reference_counting::List;
use std::cell::RefCell;
// for explicit droping
use std::mem::drop;
use std::rc::Rc;

fn main() {
    let value = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Hello, world! {:?}", value);

    // let name:&str = "love";
    let name_box = MyBox::new(String::from("Love"));

    hello(&name_box);
    
    let c = CustomerSmartPointer {
    data: String::from("my stuff"),
    };
    let _d = CustomerSmartPointer {
    data: String::from("other stuff"),
    };
    println!("CustomerSmartPointers created.");

    drop(c);


    // reference_counting
    let a = Rc::new(List::Cons(Rc::new(RefCell::new(5)), Rc::new(List::Cons(Rc::new(RefCell::new(10)), Rc::new(List::Nil)))));
    let ab = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
    
    let _ac = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
    println!("{:#?}", ab);
}

fn hello(name:&str){
    println!("Hello, my {name}!");
}
