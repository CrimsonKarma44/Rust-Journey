use traits::{
    associated_types::{Counter, Iterator},
    calling_methods_with_same_name::{
        with_self::{Human, Pilot, Wizard},
        without_self::{Animal, Dog},
    },
    generics_version,
};

fn main() {
    println!("Associated types version");
    let mut counter = Counter { count: 5 };
    while let Some(item) = counter.next() {
        println!("{}", item);
    }

    println!("Generics version");
    let mut counter = generics_version::Counter { count: 5 };
    while let Some(item) = generics_version::Iterator::<usize>::next(&mut counter) {
        println!("{}", item);
    }

    println!("Calling methods with the same name");
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);

    println!("Without self");
    println!("A baby dog is called {:?}", Dog::baby_name());
}
