use std::fmt::Display;

use crate::implementation::regular::Tweet;

use super::regular::Summary;

pub fn notify(item: &impl Summary){
   println!("Breaking News! {}", item.summarize());
}

// can implement different type
pub fn double_trait_notify(item1:&impl Summary, item2:&impl Summary){
   println!("Breaking News! {} {}", item1.summarize(), item2.summarize());
}

// implement exactly the same type
pub fn double_generic_notify<T>(item1:&T, item2:&T) -> impl Summary
where 
   T: Summary+Display
{
   println!("Breaking News! {} {}", item1.summarize(), item2.summarize());
   Tweet{
      content: String::from("value"),
      username: String::from("value"),
      reply: true,
      retweet: false,
   }
}
