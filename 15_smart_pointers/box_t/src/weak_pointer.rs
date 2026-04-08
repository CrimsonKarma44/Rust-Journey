// weak pointer
//
use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn node_tree() {
        let leaf = Rc::new(Node{
            value: 3,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![]),
        });
        // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        {
            let branch = Rc::new(Node{
                value: 5,
                parent: RefCell::new(Weak::new()),
                child: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
            assert_eq!(Rc::strong_count(&leaf), 2);
            assert_eq!(Rc::weak_count(&leaf), 0);
            assert_eq!(Rc::weak_count(&branch), 1);
        }

        assert_eq!(Rc::weak_count(&leaf), 0);
        assert_eq!(Rc::strong_count(&leaf), 1);
    }
}
