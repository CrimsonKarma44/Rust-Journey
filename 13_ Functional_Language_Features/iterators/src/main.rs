use iterators::utils::dot_iter_types::{into_iter, iter, iter_mut};
use iterators::utils::just_tests::trial;

fn main() {
    // using .iter_mut(), into_iter(), and iter()
    iter_mut();
    into_iter();
    iter();

    // running just_tests::trial()
    trial();
}
