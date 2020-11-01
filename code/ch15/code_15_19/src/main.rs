use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating  a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating  b = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("count after creating  b = {}", Rc::strong_count(&a));

    {
        let d = Cons(5, Rc::clone(&a));
        println!("count after creating  b = {}", Rc::strong_count(&a));

    }
    println!("count after droping  c = {}", Rc::strong_count(&a));

    let cc = &a;
    let dd = &a;
    let ee = &a;
    println!("--------------------  c = {}", Rc::strong_count(&a));

    let d = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("count d {}", Rc::strong_count(&d));
    let e = Cons(3, Rc::clone(&d));
    println!("count d {}", Rc::strong_count(&d));
    let f = Cons(4, Rc::clone(&d));
    println!("count d {}", Rc::strong_count(&d));
}
