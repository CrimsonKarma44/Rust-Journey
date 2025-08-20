use std::{env, fs, io};

fn main() {
    println!("Hello, world!");
    let user = User {
        username: String::from("user1"),
        password: String::from("password123"),
        id: 0,
    };

    let another_user: Box<_> = Box::new(user.clone()) as Box<dyn Handle>;

    println!("another_user username = {}", another_user.get_username());

    println!("Username: {}", user.get_username());
    let data = get_data();

    for (col, values) in data.iter() {
        println!("column: {col} => {values:?}")
    }

    let val = data.return_sublist_mean("Prices");
    println!("val = {val:?}");
}

#[allow(unused_assignments)]
fn get_data() -> Vec<Data> {
    let dir = env::current_dir().expect("Failed to get current directory");
    let filepath = dir.join("data.txt");

    let lines = fs::read_to_string(&filepath)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let mut data = Vec::new();
    let mut col = Vec::new();
    let mut name = String::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("COLUMN") {
            let header = line.split_once(" ");
            if header == None {
                continue;
            }
            name = header.unwrap().1.trim().to_string();
        } else if line.starts_with("END") {
            data.push((name.clone(), col.clone()));
            name.clear();
            col.clear();
        } else {
            col.push(line.trim().to_string());
        }
    }
    data
}

pub trait Handle {
    fn get_username(&self) -> &str;
    fn get_password(&self) -> &str;
    fn set_username(&mut self, new_username: &str);
    fn set_password(&mut self, new_password: &str);
}

pub type Data = (String, Vec<String>);

pub trait DataHandle {
    fn return_sublist_mean(&self, column: &str) -> Option<f64>;
    fn return_sublist(&self, column: &str) -> Option<Vec<String>>;
}

impl DataHandle for Vec<Data> {
    fn return_sublist(&self, column: &str) -> Option<Vec<String>> {
        self.iter()
            .find(|(col, _)| col == column)
            .map(|(_, values)| values.clone())
    }

    fn return_sublist_mean(&self, column: &str) -> Option<f64> {
        self.return_sublist(column)
            .and_then(|v| {
                let val = v
                    .iter()
                    .map(|each_v| each_v.parse::<f64>().ok())
                    .collect::<Option<Vec<f64>>>();
                val
            })
            .map(|v| (v.iter().copied().sum::<f64>(), v.len())).and_then(|v| {
                if v.1 == 0 {
                    None
                } else {
                    let mean = v.0 / (v.1 as f64);
                    let mean = format!("{mean:.0$}", 2).parse::<f64>().unwrap();
                    Some(mean)
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct User {
    username: String,
    password: String,
    id: i32,
}

impl User {
    pub fn show(&self) -> String {
        format!("id: {}, username: {}", self.id, &self.username)
    }
}

impl Handle for User {
    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_password(&self) -> &str {
        &self.password
    }

    fn set_username(&mut self, new_username: &str) {
        self.username = new_username.into();
    }

    fn set_password(&mut self, new_password: &str) {
        self.password = new_password.into();
    }
}
