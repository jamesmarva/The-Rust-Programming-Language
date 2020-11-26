pub mod blogs;
use blogs::Post;

fn main() {

    let mut post = Post::new();

    post.add_text("i ate a salad for lunch today.");
    assert_eq!("", post.get_content());


    post.request_review();
    assert_eq!("", post.get_content());


    post.approve();
    assert_eq!("i ate a salad for lunch today.", post.get_content);

}
