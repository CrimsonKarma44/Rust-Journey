#[cfg(test)]
mod tests {
    use crate::project::*;

    #[test]
    fn test_doubler() {
        let v = vec![1, 2, 3, 4, 5];
        let doubled_even = even_number_doubler(&v);
        assert_eq!(doubled_even, vec![4, 8]);
    }

    #[test]
    fn test_word_length_analyzer(){
        let word = String::from("hey newwww world");
        let word1 = String::from("Hello Mottoroller");
        let word2 = String::from("Hello world");
        assert_eq!(word_length_analyzer(&word), "newwww");
        assert_eq!(word_length_analyzer(&word1), "Mottoroller");
        assert_eq!(word_length_analyzer(&word2), "world");
    }

    #[test]
    fn test_filter_valid_username() {
        let names = vec!["Bob", "2Frank", "Ferris"];
        assert_eq!(filter_valid_username(names), vec!["Bob", "Ferris"]);
    }

    #[test]
    fn test_sum_of_square_with_fold() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_of_square_with_fold(v), Some(15));
    }

    #[test]
    fn test_find_adult_user(){
        let user = vec![

            User {
                name: String::from("Brian"),
                age: 18,
            },

            User {
                name: String::from("Dustin"),
                age: 20,
            },

            User {
                name: String::from("Greg"),
                age: 11,
            },

        ];

        let first_adult = find_first_adult_user(user);
        assert_eq!(first_adult.name, User{
            name: String::from("Dustin"),
            age: 20,
        }.name)
    }
}
