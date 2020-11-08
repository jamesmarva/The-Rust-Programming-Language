use std::rc::{Weak, Rc};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32, 
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn main() {

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!( 
        "leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf), 
        Rc::weak_count(&leaf), 
    ); 
    {
        println!("into scope");
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        // 传进去的是一个weak
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 
    
        println!( "branch strong = {}, weak = {}", 
            Rc::strong_count(&branch), 
            Rc::weak_count(&branch), 
        ); 

        println!( 
            "leaf strong = {}, weak = {}", 
            Rc::strong_count(&leaf), 
            Rc::weak_count(&leaf), 
        ); 
        println!("leave scope");
    }
    // 出来的是一个strong
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 
    println!( 
        "leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf), 
        Rc::weak_count(&leaf), 
    ); 
}
