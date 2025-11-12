pub struct Post {
    state: State,
    content: String,
}
enum State {
    Draft,
    PendingReview,
    Published,
}

impl Post{
    pub fn new() -> Post {
        Post {
            state: State::Draft,
            content: String::new(),
        }
    }
    fn add_text(&mut self, text:&str) {
        self.content.push_str(text);
    }

    fn request_review(&mut self){
        match self.state {
            State::Draft => {
                self.state = State::PendingReview
            },
            State::PendingReview => {},
            State::Published => {},
        }
    }
    fn approve(&mut self){
        match self.state {
            State::Draft => {},
            State::PendingReview => {
                self.state = State::Published;
            },
            State::Published => {},
        }
    }

    fn content(&self) -> &str {
        match self.state {
            State::Draft => "",
            State::PendingReview => "",
            State::Published => &self.content,
        }
    }
}


mod test {
    use super::*;
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