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
