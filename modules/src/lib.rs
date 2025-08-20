mod front_of_house {
   pub mod hosting {
      pub fn add_to_waitlist() {}
      fn _remove_to_waitlist() {}
   }
}

pub fn eat_at_restaurant() {
   // absolute path
   crate::front_of_house::hosting::add_to_waitlist();

   // relative path
   front_of_house::hosting::add_to_waitlist();
   // front_of_house::hosting::remove_to_waitlist();
   draw();
}
use shape::circle::draw;

mod shape{
   pub mod circle {
      pub fn draw(){
         println!("Hello World");
      }
   }
   mod square{

   }
}
