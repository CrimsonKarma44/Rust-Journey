fn main() {
    let mut value_vector:Vec<_> = Vec::<&str>::new();
    let value_box:Box<_> = Box::new([&"f", &"find"]);
    let value_box_empty:Box<[&str]> = Box::new([]);
    
    println!("vector Length before add {}", value_vector.len());
    println!("vector capacity before add: {}", value_vector.capacity());

    println!("Box Length empty: {}", value_box_empty.len());
    
    value_vector.push("f");
    
    println!("vector Length before add {}", value_vector.len());
    println!("vector capacity after add: {}", value_vector.capacity());    
    
    println!("Box Length after: {}", value_box.len());
    
    println!("Hello, world!, {:?}", value_vector);
}