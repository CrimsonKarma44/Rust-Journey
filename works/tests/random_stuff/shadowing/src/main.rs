use std::str::Bytes;

static VALUE: i32 = 49;
const ANOTHER_VALUE: i32 = 19;

fn main() {
    let decimal = 98_222;       // i32
    let hex = 0xff;             // i32 from hexadecimal
    let byte = b'A';            // u8 literal byte (ASCII 65)
    let explicit_u64 = 1_000u64;// u64 literal suffix

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("byte: {}", byte);
    println!("explicit_u64: {}", explicit_u64);
    
    let value_to_shadow = 42;
    println!("before shadowing {}", value_to_shadow);
    {
        let value_to_shadow = 32;
        println!("after shadowing {}", value_to_shadow);
    }
    println!("after shadowing {}", value_to_shadow);
    println!("static value {}", VALUE);
    println!("CONST value {}", ANOTHER_VALUE);

    println!("size of usize: {}", std::mem::size_of::<usize>());
    println!("size of Bytes: {}", std::mem::size_of::<Bytes>());
    println!("size of &str: {}", std::mem::size_of::<&str>());
    println!("size of String: {}", std::mem::size_of::<String>());
    println!("size of u8: {}", std::mem::size_of::<u8>());
    println!("size of [u8; 2]: {}", std::mem::size_of::<[u8; 2]>());
    println!("size of (u8, &str): {}", std::mem::size_of::<u32>());
    println!("size of Vec<u8>: {}", std::mem::size_of::<Vec<u8>>());
    println!("size of bool: {}", std::mem::size_of::<bool>());
}
