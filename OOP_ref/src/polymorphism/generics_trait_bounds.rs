// Using Generic and trait bounds
use crate::polymorphism::trait_polymorphic_behaviour::Draw;
pub struct Screen<T: Draw> {
    pub component: Vec<T>,
}

impl<T> Screen<T> where T: Draw {
    pub fn new (component: Vec<T>) -> Screen<T> {
        Screen{component}
    }

    pub fn run(&self) {
        for component in self.component.iter() {
            component.draw();
        }
    }
}

mod test {
    use super::Draw;
    use super::Screen;

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("Button draw for {}", self.label);
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("SelectBox draw for {}", self.options.join(", "));
        }
    }
    #[test]
    fn draw_test() {
        let list = vec![
            Button{
                width: 2,
                height: 3,
                label: String::from("OK"),
            },
            Button{
                width: 3,
                height: 43,
                label: String::from("OK NOT"),
            },
        ];
        let screen = Screen::new(list);
        screen.run();
    }
}