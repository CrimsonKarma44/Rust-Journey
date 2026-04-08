pub mod linked_list;
pub mod smart_pointer;
pub mod drop;
pub mod reference_counting;
pub mod inferior_mutability;
pub mod weak_pointer;

pub mod heap {
    #[derive(Debug)]
    pub enum List{
        Cons(i32, Box<List>),
        Nil
    }
}
