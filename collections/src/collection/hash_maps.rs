use std::collections::HashMap;

pub fn word_count(){
   println!("word count");
   let word_to_count = "hello world wonderful world";

   let mut map = HashMap::new();

   for i in word_to_count.split_whitespace(){
      let count = map.entry(i).or_insert(0);
      *count += 1;
   }
   println!("{:?}", map);
}