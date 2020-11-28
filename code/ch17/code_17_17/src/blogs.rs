trait State {
    
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>; 

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn get_val(&self) -> &str;
}

struct Draft {
    pub val: String,
}

impl Draft {
    fn new() -> Draft {
        Draft {
            val: String::from("Draft"),
        }
    }
}


impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn get_val(&self) -> &str {
        &(self.val)
    }
}

struct PendingReview {
    pub val: String,
}

impl PendingReview {
    fn new() -> Self {
        PendingReview {
            val: String::from("PendingReview"),
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published::new())
    }

    fn get_val(&self) -> &str {
        &(self.val)
    }
}

struct Published {
    pub val: String,
}

impl Published {
    fn new() -> Self {
        Published{
            val: String::from("Published"),
        }
    }
}

impl State for Published {

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn get_val(&self) -> &str {
        &(self.val)
    }
}

pub struct Post {
    pub content: String, 
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Self {
        Post {
            content: String::from(""),
            state: Some(Box::new(Draft::new())),
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn content(&self) -> &str {
        // self.state.as_ref().unwrap().content(self);
        // let tmp: &Box<dyn State> = self.state.as_ref().unwrap();
        // return tmp.content(self);
        let tmp: Box<dyn State> = self.state.unwrap();
        tmp.content(self)
    }

    pub fn check_state(&self) -> &str {
        self.state.as_ref().unwrap().get_val()
    }
}