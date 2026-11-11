use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    // assert_eq!(size_of_val(&c1),__);
    println!("{}", size_of_val(&c1));
    let c2 = '中';
    // assert_eq!(size_of_val(&c2),__);
    println!("{}", size_of_val(&c2));
    println!("Success!")
}
