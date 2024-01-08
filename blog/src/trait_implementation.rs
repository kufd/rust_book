pub fn run() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("Blog State: {}", post.get_state());

    post.request_review();
    assert_eq!("", post.content());
    println!("Blog State: {}", post.get_state());

    post.reject();
    assert_eq!("", post.content());
    println!("Blog State: {}", post.get_state());

    post.request_review();
    assert_eq!("", post.content());
    println!("Blog State: {}", post.get_state());

    post.approve();
    assert_eq!("", post.content());
    println!("Blog State: {}", post.get_state());

    post.add_text("MORE");
    assert_eq!("", post.content());
    println!("Blog State: {}", post.get_state());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Blog State: {}", post.get_state());
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        let state = self.state.as_ref().unwrap();

        if state.can_add_text() {
            self.content.push_str(text);
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    fn get_state(&self) -> String {
        self.state.as_ref().unwrap().get_name()
    }
}

trait State {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn can_add_text(&self) -> bool {
        false
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn get_name(&self) -> String;
}

// --snip--

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_name(&self) -> String {
        String::from("Draft")
    }
}

struct PendingReview {
    approves_number: u8
}

impl PendingReview {
    fn new() -> PendingReview {
        PendingReview {
            approves_number: 0
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approves_number += 1;

        if self.approves_number >= 2 {
            return Box::new(Published {});
        }

        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn get_name(&self) -> String {
        String::from("PendingReview")
    }
}

struct Published {}

impl State for Published {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_name(&self) -> String {
        String::from("Published")
    }
}