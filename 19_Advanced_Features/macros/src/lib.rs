// they are like the maatch expression
pub mod declarative_macro {
    #[macro_export]
    macro_rules! myVec {
        ( $($x:expr),* $(,)?) => {
            {
                let mut vector = Vec::new();
                $(
                    vector.push($x);
                )*
                vector
            }
        };
    }
}

pub mod procedural_macros {
    use proc_macro;

    pub mod custom {
        pub trait HelloMacro {
            fn hello_macro();
        }
    }

    pub mod attribute_like {}

    pub mod function_like {}
}
