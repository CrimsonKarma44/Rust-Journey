fn main() {
    let mut word1 = String::from("abc");
    println!("{}", word1);
    closure_test(|x| {
        word1.push_str(x);
        x.push_str("defg");
        x.len()
    });
}

fn closure_test(mut value: impl FnMut(&mut String) -> usize) {
    let mut item = "abc".to_string();
    let length = value(&mut item);
    println!("{}", length);
}
