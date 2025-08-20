use std::{fs::File, io::{self, ErrorKind}};

pub fn unrecoverable_error(){
    panic!("I have no choice but to panic here...");
}

pub fn recoverable_error(){
   let greeting_file_result = File::open("hello.txt");
   match greeting_file_result {
       Ok(_t) => println!("file found!"),
       Err(e) => match e.kind() {
          ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(_fc) => println!("file not found but created!"),
            Err(err) => panic!("{:}", err),
         },
         other_error => panic!("{:}", other_error),
       },
   }
}

pub fn another_way_recoverable() -> File {
   let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
         File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
         })
      } else {
         panic!("Problem opening the file: {:?}", error);
      }
   });
   greeting_file

}


pub fn test(value:Option<i32>) -> Result<i32, io::Error> {
   let custom_error = io::Error::new(ErrorKind::NotFound, "Not present");
   match value {
      Some(t) => Ok(t),
      None => Err(custom_error),
   }
}