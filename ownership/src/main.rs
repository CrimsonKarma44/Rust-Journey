fn main() {
    let mut s = String::from("hello");
    println!("{s}"); // This will print `hello, world!`
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`
    
    let s1 = s.clone();
    println!("{s1}"); // This will print `hello, world!`
    println!("{s}"); // This will print `hello, world!`
    mutable_reference();
    println!("{}", slice_reference(&s));
    test();
}

fn mutable_reference(){
    println!("Mutable reference");
    let mut s = String::from("Love");
    let s1 = &mut s;
    println!("{s1}");
    let s2 = &mut s;
    println!("{s2}");
}

fn slice_reference(value: &str)->  &str{
    println!("Slice reference");
    println!("{}", value);
    let bytes = value.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &value[..i];
        }
    }   
   &value[..]
}

fn test(){
    println!("Test");
    let my_string = String::from("hello world");
    // `slice_reference` works on slices of `String`s, whether partial or whole
    let word = slice_reference(&my_string[0..6]);
    println!("{}", word);
    let word = slice_reference(&my_string[..]);
    println!("{}", word);
    // `slice_reference` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = slice_reference(&my_string);
    println!("{}", word);
    let my_string_literal = "hello world";
    // `slice_reference` works on slices of string literals, whether partial or whole
    let word = slice_reference(&my_string_literal[0..6]);
    println!("{}", word);
    let word = slice_reference(&my_string_literal[..]);
    println!("{}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = slice_reference(my_string_literal);
    println!("{}", word);
}