use std::rc::Rc;
use std::cell::RefCell;
// allows only immmutable borrows

#[derive(Debug)]
pub enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
