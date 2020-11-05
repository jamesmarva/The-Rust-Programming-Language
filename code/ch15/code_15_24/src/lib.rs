use std::rc::Rc;
use std::cell::RefCell;

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[]

