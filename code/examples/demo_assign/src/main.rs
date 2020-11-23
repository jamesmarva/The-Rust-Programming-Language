fn main() {

    let mut x = ListNode {
        val: 1,
        next: None,
    };
    x = ListNode {
        val: 2,
        next: None,
    };

    let mut dummy = ListNode::new(0);
    dummy.next = Some(Box::new(x));
    let mut f = &dummy;
    let mut s = &dummy;
    
    // cannot move out of `f.next` 
    // which is behind a shared reference

    // move occurs because `f.next` has type 
    //`std::option::Option<std::boxed::Box<ListNode>>`,
    //  which does not implement the `Copy` trait
    // f = &(*(f.next.unwrap()));
    
    f = &(*(f.next.as_ref().unwrap()));
    f = &(*(f.next.as_ref().unwrap()));
    // if 
    s = &(*(s.next.as_ref().unwrap()));


    
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
      ListNode {
        next: None,
        val
      }
    }
  }