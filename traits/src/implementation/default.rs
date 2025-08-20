use std::fmt::Write;

pub trait Summari {
    fn summarize(&self) -> String {
      String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summari for NewsArticle {
    fn summarize(&self) -> String {
        let mut sentence = String::new();
        sentence.write_fmt(format_args!("{}", self.headline)).unwrap();
        sentence
    }
}