mod linked_list;

pub mod heap {
    #[derive(Debug)]
    pub enum List{
        Cons(i32, Box<List>),
        Nil,
    }
}

pub fn luck(){
    println!("Hello, world!");
}