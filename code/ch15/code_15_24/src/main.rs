use std::rc::Rc;
use std::cell::RefCell;

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>), 
//     Nil,
// }
// 多个持有的可变的对象

use code_15_24::List::{Cons, Nil};

fn main() {

    let v = Rc::new(RefCell::new(55));

    let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(11)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(22)), Rc::clone(&a));

    *v.borrow_mut() += 10;
    
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}
