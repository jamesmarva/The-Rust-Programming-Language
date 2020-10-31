
use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use code_15_18::ListNode;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let bn = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("aa");


    let mut a = ListNode::new(1);
    println!("a : {}", a.val);
    a.next = Some(Box::new(ListNode::new(2)));
    a = *a.next.unwrap();
    println!("a : {}", a.val);

    a.next = Some(Box::new(ListNode::new(3)));
    a = *a.next.unwrap();
    println!("a : {}", a.val);
    
}

