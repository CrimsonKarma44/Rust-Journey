
pub fn looper(array: &mut Vec<i32>){
    println!("vactor looper version");
    for i in array{
        println!("{}", i);
        *i += 10;
        println!("{}", i);
    }
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn storing_multiple_datatypes(){
    println!("Storing multiple datatypes");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        ];
    for i in row{
        match i {
            SpreadsheetCell::Float(val) => println!("{}", val),
            SpreadsheetCell::Int(val) => println!("{}", val),
            SpreadsheetCell::Text(val) => println!("{}", &val),
        }
    }
}