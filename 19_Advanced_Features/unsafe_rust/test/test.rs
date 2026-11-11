// use crate::raw_pointer::{creating_from_reference, splitting_slice_ptr};
use unsafe_rust::extern_functions::calling_abs;
use unsafe_rust::mut_static::{get_counter, get_hello_world, increment_counter};
use unsafe_rust::raw_pointer::{creating_from_reference, splitting_slice_ptr};

#[test]
fn creating_reference() {
    let mut value = 3;

    creating_from_reference(&mut value);
    assert_eq!(value, 4);
}

#[test]
fn splitting_arry_by_ptr_test() {
    let mut array = [1, 2, 3, 4, 5];

    let (left, right) = splitting_slice_ptr(&mut array, 2);

    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5]);
}

#[test]
fn calling_abs_test() {
    assert_eq!(calling_abs(-1), 1);
    assert_eq!(calling_abs(1), 1);
}

#[test]
fn mut_static_test() {
    assert_eq!(get_hello_world(), "Hello, World!");
    assert_eq!(get_counter(), 0);
    increment_counter();
    assert_eq!(get_counter(), 1);
}
