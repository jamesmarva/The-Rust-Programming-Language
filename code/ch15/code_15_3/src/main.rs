use crate::List::{Cons, Nil}; 

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {

    let list = Cons(1, Cons(3, Nil));

}


