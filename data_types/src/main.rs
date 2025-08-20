fn main() {
    // type conversion
    let value : u32 = "42".parse().expect("Not a Number!");
    println!("{}", value);
    
    let number = 98_222;
    println!("{}", number);
    
    // interger overflow
    // let _overflow : u8 = 256;
    // println!("{}", _overflow);

    // floats
    let _x = 2.0;
    let _y : f32 = -3.0; 

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 5 % 3; // Results in -1
    println!("{}, {} and {}", quotient, truncated, remainder);

    // tuple
    let tup = (500, 6.4, 1, true);
    let (x, y, z, conditional) = tup;
    println!("{} {} {} {}", x, y, z, conditional);
    println!("{} {} {} {}", tup.0, tup.1, tup.2, tup.3);// using their posiiton

    // arrays
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    let a  = [3; 5];
    
    // function
    another_function(a);
    loop_function();
    conditional_statement();
    println!("{}", test_function());
    for_loop();
}

fn for_loop(){
    println!("For loop");
    //(1..6).rev() for reverse
    for number in 1..6 {
        println!("{number}");
    }
}

fn another_function(a:[i32; 5]){
    println!("Another Function");
    println!("{}", a[0]);
}

fn conditional_statement(){
    let number = 3;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("{}", number);
}

fn loop_function(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
fn test_function()-> i32{
    println!("Test Function");
    let x = 3;
    let y = {
        x
    };
    println!("{}", y);
    println!("{}", x);
    y
}
