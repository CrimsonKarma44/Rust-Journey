pub mod associated_types {
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    pub struct Counter {
        pub count: usize,
    }
    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some(self.count)
            }
        }
    }
}

pub mod generics_version {
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    pub struct Counter {
        pub count: usize,
    }
    impl Iterator<usize> for Counter {
        fn next(&mut self) -> Option<usize> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some(self.count)
            }
        }
    }
}

pub mod default_generic_type_params {
    use std::ops::Add;
    pub mod defaulted {
        use super::Add;

        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct Point<T = usize> {
            pub x: T,
            pub y: T,
        }
        impl Add for Point {
            type Output = Point;
            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
    }

    pub mod overridden {
        use super::Add;

        pub struct Meters(pub u32);

        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct MiliMeters(pub u32);

        impl Add<Meters> for MiliMeters {
            type Output = MiliMeters;
            fn add(self, other: Meters) -> MiliMeters {
                MiliMeters(self.0 + other.0 * 1000)
            }
        }
    }
}

pub mod calling_methods_with_same_name {
    pub mod with_self {
        pub trait Wizard {
            fn fly(&self);
        }
        pub trait Pilot {
            fn fly(&self);
        }
        pub struct Human;

        impl Human {
            pub fn fly(&self) {
                println!("i can fly... the best")
            }
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("broom flying... the best")
            }
        }
        impl Pilot for Human {
            fn fly(&self) {
                println!("Airplains... the best")
            }
        }
    }
    pub mod without_self {
        pub trait Animal {
            fn baby_name() -> String;
        }

        pub struct Dog;

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("Puppy")
            }
        }
    }
}
