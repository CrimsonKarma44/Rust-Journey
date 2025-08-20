#[derive(Debug)]
pub struct Linkedlist<T>{
    head: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T>{
    data: T,
    next: Link<T>,
}

impl<T> Linkedlist<T>{
    pub fn new() -> Linkedlist<T>{
        Linkedlist{head: None}
    }

    pub fn push(&mut self, data: T){
        let new_node = Box::new(Node{data, next: self.head.take()});
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T>{
        match self.head.take(){
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None
        }
    }
}

mod test{
    #[test]
    fn test_node(){
        use super::Linkedlist;
        let mut node = Linkedlist::new();
        node.push(1);
        node.push(2);
        let mut list = node;
        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }
}