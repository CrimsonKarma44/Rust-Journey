pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post{
    pub fn new() -> Post{
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str){
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text(self, text));
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref()
            .unwrap()
            .content(self)
    }
    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(self: Box<Self>, post: &mut Post, text: &str) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str{
        ""
    }
}
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview { approval_score: 0})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn add_text(self: Box<Self>, post: &mut Post, text: &str) -> Box<dyn State> {
        post.content = text.to_string();
        self
    }
}

struct PendingReview {
    approval_score: u32,
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.approval_score < 1 {
            self.approval_score += 1;
            self
        } else {
            self.approval_score = 0;
            Box::new(Published {})
        }
    }
    fn add_text(self: Box<Self>, _post: &mut Post, text: &str) -> Box<dyn State>{
        self
    }
    fn reject(mut self: Box<Self>) -> Box<dyn State> {
        self.approval_score = 0;
        Box::new(Draft {})
    }
}
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn add_text(self: Box<Self>, _post: &mut Post, text: &str) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        &post.content
    }
}

mod test {
    use std::time::Instant;
    use super::*;

    #[test]
    fn it_works() {
        let now = Instant::now();
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
        println!("elapsed: {:?}", now.elapsed());
    //     3.924µs
    }

    #[test]
    fn reject_test(){
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.request_review();
        post.add_text("I ate a salad for lunch today two");
        assert_eq!("", post.content());
        post.approve();
        assert_ne!("I ate a salad for lunch today", post.content());
        post.reject();
        assert_ne!("I ate a salad for lunch today", post.content());
        post.request_review();
        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

