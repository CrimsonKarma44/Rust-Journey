use merge_strings_alternately_1768::{my_solution, others_solution};

fn main() {
    println!(
        "{}",
        my_solution::merge_alternately(String::from("abc"), String::from("pqgr"))
    );
    println!(
        "{}",
        others_solution::merge_alternately(String::from("abc"), String::from("pqgr"))
    );
}
