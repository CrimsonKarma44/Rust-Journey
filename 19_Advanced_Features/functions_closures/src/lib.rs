pub mod advanced_functions_closures {
    pub mod fn_function_pointers {
        pub fn add_one(value: i32) -> i32 {
            value + 1
        }
        
        pub fn do_twice(value: i32, f: impl Fn(i32) -> i32) -> i32 {
            f(value) + f(value)
        }
        
        pub fn do_twice_with_generics<F>(value: i32, f: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            f(value) + f(value)
        }
        
        #[derive(Debug, PartialEq)]
        pub enum Status {
            Value(u32),
            Stop,
        }
        pub struct Accept {
            value: u32,
        }
        impl Accept {
            pub fn new(value: u32) -> Self {
                Self { value }
            }
            pub fn conv_status(&self, f: impl Fn(u32) -> Status) -> Status {
                f(self.value)
            }
        }
        
    }
    pub mod returing_closures {
        pub fn returning_closure_impl() -> impl Fn() -> u32 {
            || 42
        }
        pub fn returning_closure_from_fn() -> impl Fn() -> u32 {
            fn closure() -> u32 {
                42
            }
            closure
        }
        pub fn returning_closure_from_fn_mut() -> Box<dyn Fn() -> u32> {
            fn closure() -> u32 {
                42
            }
            Box::new(closure)
        }
        pub fn returning_fn_from_fn_casted() -> fn() -> u32 {
            fn closure() -> u32 {   
                42
            }
            closure as fn() -> u32
        }
        pub fn returning_closure_from_fn_casted() -> fn() -> u32 {
            || 42
        }
        
    }
    
}

pub mod experiment {
    pub fn closure_test(mut value: impl FnMut(&mut String) -> usize) -> usize {
        let mut item = "abc".to_string();
        let length = value(&mut item);
        println!("{}", length);
        length
    }
}