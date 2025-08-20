pub mod default{
   #[derive(Debug, PartialEq, Copy, Clone)]
   enum ShirtColor {
      Red,
      Blue,
   }
   struct Inventory {
      shirts: Vec<ShirtColor>,
   }
   impl Inventory {
      fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
         user_preference.unwrap_or_else(|| self.most_stocked())
      }

      fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
      }
   }
   pub fn default_test(){
      let store = Inventory {
          shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
      };
      let user_pref1 = Some(ShirtColor::Red);
      let giveaway1 = store.giveaway(user_pref1);
      println!(
          "The user with preference {:?} gets {:?}",
          user_pref1, giveaway1
      );
      let user_pref2 = None;
      let giveaway2 = store.giveaway(user_pref2);
      println!(
          "The user with preference {:?} gets {:?}",
          user_pref2, giveaway2
      );
  }
}
pub mod borrowing_mutably{
   pub fn test_borrowing_mutably() {
      let list = vec![1, 2, 3];
      println!("Before defining closure: {list:?}");
      let only_borrows = || println!("From closure: {list:?}");
      println!("Before calling closure: {list:?}");
      only_borrows();
      println!("After calling closure: {list:?}");
   }
}

struct Array<T>(Vec<T>);

impl <T> Array<T> {
    fn new() -> Self {
        Array(Vec::new())
    }

    fn push(&mut self, value: T) {
        self.0.push(value);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        self.0.sort();
    }

    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.0.sort_by(compare);
    }

}

pub mod borrowing_immutably{
   use super::Array;
   pub fn test_borrowing_immutably() {
      let mut list = vec![1, 2, 3];
      println!("Before defining closure: {list:?}");
      let mut borrows_mutably = || list.push(7);
      borrows_mutably();
      println!("After calling closure: {list:?}");
   }
    pub fn test_array(){
         let mut array = Array::new();
         array.push(1);
         array.push(2);
         array.push(3);
         println!("Array after pushing elements: {:?}", array.0);

         if let Some(value) = array.get(1) {
               println!("Element at index 1: {}", value);
         } else {
               println!("No element at index 1");
         }

         array.sort();
         println!("Array after sorting: {:?}", array.0);

         array.sort_by(|a, b| b.cmp(a));
         println!("Array after sorting in descending order: {:?}", array.0);

   }
}
pub mod taking_ownership{
    use std::thread;

   pub fn test_taking_ownership() {
      let list = vec![1, 2, 3];
      println!("Before defining closure: {list:?}");
      thread::spawn(move || println!("From thread: {list:?}"))
      .join()
      .unwrap();
   }
}

