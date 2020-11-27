
trait State {}


struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}


impl Post {

    pub fn content(&self) -> &str {
        ""
    }
}

struct Draft {}

impl State for Draft {}


struct PendingReview {}

impl State for PendingReview {}


struct Published {}

impl State for Published {}

