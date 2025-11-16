pub struct Post {
    state: Option<State>,
    content: String,
}

#[derive(Debug)]
enum State {
    Draft,
    PendingReview,
    Published,
}
impl State {
    fn request_review(self) -> State {
        match self {
            State::Draft => {
                State::PendingReview
            },
            State::PendingReview => self,
            State::Published => self,
        }
    }
    fn approve(self) -> Self {
        match self {
            State::Draft => self,
            State::PendingReview => {
                State::Published
            },
            State::Published => self,
        }
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        match self {
            State::Draft => "",
            State::PendingReview => "",
            State::Published => &post.content,
        }
    }
}

impl Post{
    pub fn new() -> Post {
        Post {
            state: Some(State::Draft),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text:&str) {
        self.content.push_str(text);
    }
    pub fn request_review(&mut self){
         self.state = Some(self.state
             .take()
             .unwrap()
             .request_review());
    }
    pub fn approve(&mut self){
        self.state = Some(self.state
            .take()
            .unwrap()
            .approve())
    }
    pub fn content(&self) -> &str {
        self.state
            .as_ref()
            .unwrap()
            .content(&self)
    }
}


mod test {
    use std::time::Instant;
    use super::*;
    #[test]
    fn random_test(){
        let value = vec![State::Published, State::Draft, State::PendingReview];
        for state in value.iter(){
            println!("state: {:?}", state);
        }
    }
    #[test]
    fn it_works() {
        let now = Instant::now();
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
        println!("elapsed: {:?}", now.elapsed());
        //    2.434µs
    }
    #[test]
    fn post_approve_before_review_test() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.request_review();
        assert_ne!("I ate a salad for lunch today", post.content());
    }
}