#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        use super::super::methods::method_consuming_iter;
        let v1 = vec![1, 2, 3];
        let total = method_consuming_iter(&v1);

        assert_eq!(total, 6);
    }

    #[test]
    fn method_iterators() {
        use super::super::methods::{method_consuming_iter, method_producing_iters};
        let new_array = method_producing_iters();
        assert_eq!(new_array, vec![3, 4, 5, 6, 7]);
        let v1 = vec![1, 2, 3];
        let total = method_consuming_iter(&v1);
        assert_eq!(total, 6);
    }

    struct Shoes {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoes>, size: u32) -> Vec<Shoes> {
        shoes.into_iter().filter(|s| s.size == size).collect()
    }

    #[test]
    fn iterator_filter() {
        let shoes = vec![
            Shoes {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoes {
                size: 12,
                style: String::from("sandal"),
            },
            Shoes {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(in_my_size.len(), 2);
        assert_eq!(in_my_size[0].style, "sneaker");
    }

    #[test]
    fn testing_iter_loop(){
        use super::super::just_tests::looping_iter;

        let v1 = vec![1, 2, 3];
        looping_iter(&v1);
    }
}
