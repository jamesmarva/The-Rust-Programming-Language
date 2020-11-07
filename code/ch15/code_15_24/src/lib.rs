
#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;

mod tests {
    use super::*;
    use super::List::{Cons, Nil};

    #[test]
    fn test() {
        println!("aaaaaaaa");
        let val = Rc::new(RefCell::new(5));
        
        let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(11)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(22)), Rc::clone(&a));

        *(val.borrow_mut()) += 100;
        
        println!("a after {:?}", a);
        println!("b after {:?}", b);
        println!("c after {:?}", c);
        assert_eq!("a", "a");
    }
}
