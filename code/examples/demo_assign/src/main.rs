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
    f = &*(f.next.unwrap());

    
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