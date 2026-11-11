fn main() {
    let username = String::from("value");
    let email = String::from("value@email.com");
    let mut new_user_1 = create_user(username, email);
    dbg!(&new_user_1);

    
    
    let username = String::from("Darksummer");
    let email = String::from("Darksummere@email.com");
    let new_user_2 = copy_user_state(username, email, &new_user_1);
    // let new_user_2 = User{
    //     username,
    //     email,
    //     ..new_user_1
    // };
    
    new_user_1.increment_active_count();

    println!("{new_user_1:#?}");
    println!("{1} {0}", new_user_1.username, new_user_1.email);
    println!("{new_user_2:#?}");

    let cat_1 = Animal{};
    println!("{cat_1:#?}");

    Animal::meow();
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    active_in_count: u64,
}
impl User{ 
    fn increment_active_count(&mut self) {
        self.active_in_count += 1;
    }
}
fn create_user(username: String, email: String)-> User {
    User {
        active: true,
        username,
        email,
        active_in_count: 1,
    }
}

fn copy_user_state(username: String, email: String, inherit_user: &User) -> User {
    User{
        username,
        email,
        active: inherit_user.active,
        active_in_count: inherit_user.active_in_count,
    }
}

#[derive(Debug)]
struct Animal{}

impl Animal{
    fn meow(){
        println!("Meow!");
    }
}