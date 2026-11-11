pub mod match_arm{
    pub fn regular_match(x: Option<u32>) {
        match x {
            None => println!("None"),
            Some(x) => match x {
                1 => println!("one"),
                2 => println!("two"),
                3 => println!("three"),
                _ => println!("anything"),
            },
        }
    }

    pub fn scoped_definition() {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }
        println!("at the end: x = {x:?}, y = {y}");
    }

    pub fn mutltiple_pattern(){
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything",)
        }
    }

    pub fn matching_ranges_of_values(){
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    /// adding extra condition to the match condition
    pub fn match_guard(){
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
    }

    #[derive(Debug)]
    enum Message{
        Hello {id:i32},
    }
    pub fn bindings(id:i32){
        let msg = Message::Hello {id};

        match msg {
            Message::Hello { id: id_variable @ 3..=7 } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}



pub mod loops {
    pub fn while_loops(){
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop(){
            println!("{top}");
        }
    }

    pub fn for_loops(){
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

}

pub mod scoped_structures {
    pub fn func_matching(){
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({x}, {y})");
        }

        print_coordinates(&(0, 0));
    }

    struct Point{
        x:i32,
        y:i32,
    }
    pub fn struct_matching(){
        let p = Point{
            x:0,
            y: 7,
        };

        let Point {x:a, y:b} = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    pub fn match_struct(){
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    pub fn deconstruct_enum(){
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
    }
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum AnotherMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    pub fn nested_enum_struct() {
        let msg = AnotherMessage::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            AnotherMessage::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            AnotherMessage::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }
}
