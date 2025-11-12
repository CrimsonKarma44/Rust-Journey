pub struct Post {
    content: String,
    in_review: bool,
    approved: bool,
}
impl Post {
    pub fn new() -> Post {
        Post{content: String::new(), in_review: false, approved: false}
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        self.in_review = true;
    }

    pub fn approve(&mut self) {
        if self.in_review {
            self.approved = true;
        } else { self.approved = false; }
    }
    pub fn content(&self) -> &str {
        if self.in_review && self.approved {
            &self.content
        } else {
            ""
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn post_test() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
    #[test]
    fn post_approve_before_review_test() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.request_review();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

