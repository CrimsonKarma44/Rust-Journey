use std::fmt::Display;

pub trait ToString {
    fn __string__(&self) -> String{
      format!("This is a stringer implementation...")
    }
}

impl<T: Display> ToString for T {
// --snip--
}