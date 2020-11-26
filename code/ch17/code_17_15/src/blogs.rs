
pub struct Post {
    content: String, 
    state: Option<Box<dyn State>>
}

impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn add_txt(&mut self, s: String) {
        self.content.push_str(&s);
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; 
    fn appprove(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
     
    fn appprove(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn appprove(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn appprove(self: Box<Self>) -> Box<dyn State> {
        self
    }
}








