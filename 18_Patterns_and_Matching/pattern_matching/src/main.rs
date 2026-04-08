use pattern_matching::match_arm::bindings;
fn main() {
    println!("Hello, world!");

    let mut value = Some(10);
    bindings(value.unwrap());

    while  let x @ 5..=10 = value.unwrap() {
        println!("x:{x}, value:{value:?}");
        value = Some(value.unwrap() - 1);
    }

    let grouped = vec!["a", "b", "c"];
    for x in 'a'..'z' {
        let x_str = x.to_string();
        if grouped.contains(&x_str.as_str()) {
            println!("Found a match: {x_str}");
        }
    }
}
