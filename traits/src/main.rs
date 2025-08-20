use std::env;
use implementation::regular::{Summary, Tweet};
use implementation::default::{Summari, NewsArticle};
use implementation::parameter::{notify, double_generic_notify, double_trait_notify};

// Note trait in rust is similar to interface in golang but in their usage and concept

pub mod implementation;
fn main() {
    let value = env::args().collect::<Vec<String>>();
    println!("{:?}", env::args());
    println!("{:?}", value);
    test_default();
    test_regular();

}
fn test_default(){
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}

fn test_regular(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}