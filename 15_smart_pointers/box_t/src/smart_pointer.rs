use std::ops::Deref;

pub struct MyBox<T>(T);
impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

mod tests{

    #[test]
    fn type_assert(){
        use crate::smart_pointer::MyBox;
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(x, *y);
    }
}
