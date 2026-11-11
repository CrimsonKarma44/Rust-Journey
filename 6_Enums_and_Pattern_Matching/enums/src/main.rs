fn main() {
    let rectangle = Shape::Rectangle(3.0, 2.0);
    let circle = Shape::Circle(3.4);
    
    println!("area of the rectangle {}", rectangle.area());
    println!("perimeter of the rectangle {}", rectangle.perimeter());
    println!("area of the circle {}", circle.area());
    println!("perimeter of the circle {}", circle.perimeter());
    rectangle.__string__();
    
    let value = Some(40);
    println!("{}", value.unwrap() == 40);
}


enum Shape {
    Rectangle(f32, f32),
    Circle(f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Rectangle(length, breadth) => {
                let l = length;
                let b = breadth;
                l*b
            },
            Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
        }
    }
    
    fn perimeter(&self) -> f32 {
        match self {
            Shape::Rectangle(length, breadth) => 2.0 * (length*breadth),
            Shape::Circle(radius) => std::f32::consts::PI * radius * 2.0,
        }
    }

    fn __string__(&self) {
        match self {
            Shape::Rectangle(length, breadth) => println!("A Rectangle: length {length}, breath {breadth}"),
            Shape::Circle(radius) => println!("A Circle: radius {}", *radius),
        }
    }
}
