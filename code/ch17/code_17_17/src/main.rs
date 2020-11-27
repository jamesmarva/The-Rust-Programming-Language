pub mod blogs;

use blogs::Post;


fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    assert_eq!("Draft", post.check_state());

    post.approve(); 
    assert_eq!("", post.content());
    assert_eq!("PendingReview", post.check_state());

    post.approve(); 
    assert_eq!("Published", post.check_state());
    assert_eq!("I ate a salad for lunch today", post.content()); 

}
