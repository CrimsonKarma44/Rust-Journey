mod type_synonym {
    use std::io::Error;

    pub type Synonym = i32;

    pub type thunk = Box<dyn Fn() + Send + 'static>;

    pub mod without_alias {
        use super::Error;
        pub trait WriteWithoutAlias {
            fn write(&self, buf: &[u8]) -> Result<usize, Error>;
            fn flush(&self) -> Result<(), Error>;

            fn write_all(&self, buf: &[u8]) -> Result<(), Error>;
            fn write_fmt(&self, fmt: std::fmt::Arguments) -> Result<(), Error>;
        }
    }
    pub mod with_alias {
        use super::Error;
        pub type Result<T> = std::result::Result<T, Error>;
        pub trait WriteWithAlias {
            fn write(&self, buf: &[u8]) -> Result<usize>;
            fn flush(&self) -> Result<()>;

            fn write_all(&self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&self, fmt: std::fmt::Arguments) -> Result<()>;
        }
    }
}

pub mod never_type_never_return {}
pub mod dynamic_sized_traits_types {}
