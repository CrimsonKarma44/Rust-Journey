// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of a union

pub mod raw_pointer {
    use std::slice;
    pub fn creating_from_reference(num: &mut i32) {
        let _r1 = num as *const i32;
        let r2 = num as *mut i32;

        unsafe {
            // dereferencing: viewing and modifying can only be done in an unsafe block
            *r2 += 1;
        }
    }

    pub fn unsafe_function() {
        unsafe fn dangerous() {}
        unsafe {
            dangerous();
        }
    }

    pub fn splitting_slice_ptr(array: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let ptr = array.as_mut_ptr();
        let length = array.len();

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), length - mid),
            )
        }
    }
}

/// use for calling external functions from other programming languages
pub mod extern_functions {
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    pub fn calling_abs(input: i32) -> i32 {
        unsafe { abs(input) }
    }
}

pub mod mut_static {
    static HELLO_WORLD: &str = "Hello, World!";
    static mut COUNTER: u32 = 0;

    pub fn get_hello_world() -> &'static str {
        HELLO_WORLD
    }

    pub fn get_counter() -> u32 {
        unsafe { COUNTER }
    }

    pub fn increment_counter() {
        unsafe {
            COUNTER += 1;
        }
    }
}

pub mod unsafe_trait {
    pub unsafe trait Foo {}
    unsafe impl Foo for i32 {}
}

mod test {
    // use crate::raw_pointer::{creating_from_reference, splitting_slice_ptr};
    use super::raw_pointer::{creating_from_reference, splitting_slice_ptr};

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

    use super::extern_functions::calling_abs;

    #[test]
    fn calling_abs_test() {
        assert_eq!(calling_abs(-1), 1);
        assert_eq!(calling_abs(1), 1);
    }

    use super::mut_static::{get_counter, get_hello_world, increment_counter};
    #[test]
    fn mut_static_test() {
        assert_eq!(get_hello_world(), "Hello, World!");
        assert_eq!(get_counter(), 0);
        increment_counter();
        assert_eq!(get_counter(), 1);
    }
}
