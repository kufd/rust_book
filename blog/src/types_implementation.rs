pub fn run() {
    let mut post = Post::new();
    println!("Blog State: {}", post.get_state());

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    println!("Blog State: {}", post.get_state());

    let post = post.approve();
    println!("Blog State: {}", post.get_state());

    assert_eq!("I ate a salad for lunch today", post.content());
}

struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn get_state(&self) -> String {
        String::from("Published")
    }
}

impl DraftPost {
    // --snip--
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }

    fn get_state(&self) -> String {
        String::from("Draft")
    }
}

struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    fn get_state(&self) -> String {
        String::from("PendingReview")
    }
}