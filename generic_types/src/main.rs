use generics::{functions::largest, structs::Point};

pub mod generics;
fn main() {
    test_struct();
    test_function();
}

fn test_struct(){
    println!("test struct");
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


fn test_function() {
    println!("test function");
   let number_list = vec![34, 50, 25, 100, 65];
   let result = largest(&number_list);
   println!("The largest number is {result}");
   let char_list = vec!['y', 'm', 'a', 'q'];
   let result = largest(&char_list);
   println!("The largest char is {result}");
}