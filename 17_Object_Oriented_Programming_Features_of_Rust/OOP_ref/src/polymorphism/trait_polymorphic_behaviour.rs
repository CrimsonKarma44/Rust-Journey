pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new (components: Vec<Box<dyn Draw>>) -> Screen {
        Screen { components }
    }
    pub fn run(&self) {
        for component in self.components.iter() {
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
        let list:Vec<Box<dyn Draw>> = vec![
            Box::new(Button{
                width: 2,
                height: 3,
                label: String::from("OK"),
            }),
            Box::new(SelectBox{
                width: 3,
                height: 3,
                options: vec![String::from("Yes"), String::from("Maybe")],
            }
            ),
        ];
        let screen = Screen::new(list);
        screen.run();
    }
}