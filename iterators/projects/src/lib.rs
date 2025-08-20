mod test;

pub mod project{
    /// This function doubles any even number in a vector
    /// ```
    ///     #[test]
    ///     fn test_doubler() {
    ///         let v = vec![1, 2, 3, 4, 5];    
    ///         let doubled_even = even_number_doubler(&v);
    ///         assert_eq!(doubled_even, vec![4, 8]);
    ///     }
    /// ```
    pub fn even_number_doubler(value: &Vec<i32>) -> Vec<i32> {
        value.iter().filter(|&x| *x % 2 == 0).map(|&x| x*2).collect::<Vec<i32>>()
    }

    /// only works for the last not the first match
    pub fn word_length_analyzer(word: &str) -> &str {
        word.split_whitespace().into_iter().map(|x| (x.len() as i32, x)).max().unwrap().1
    }

    pub fn filter_valid_username(names: Vec<&str>) -> Vec<&str> {
        names.into_iter().filter(|&x| x.len() >= 3 &&  x.chars().next().unwrap().is_alphabetic()).collect()
    }

    /// equivalent to the reduce in python
    ///
    /// can also be referred to as an accumulator
    pub fn sum_of_square_with_fold(store: Vec<i32>) -> Option<i32>{
        Some(store.iter().fold(0, |acc, x| acc + x))
    }
   pub struct User {
        pub name: String,
        pub age: u8,
    }
    pub fn find_first_adult_user(store: Vec<User>) -> User {
        store.into_iter().find(|x| x.age > 18).unwrap()
    }
}
