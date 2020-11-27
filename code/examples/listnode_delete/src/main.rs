#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32, 
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(&self, val: i32) -> Self {
        ListNode{
            next: None, 
            val,
        }
    }
}

fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut head_clone = head.clone();
    let mut slow = &mut head_clone;
    let mut fast = &mut head;
    let mut i = 1;
    while i < n {
        fast = &mut fast.as_mut().unwrap().next;
        i+=1;
    }    

    while fast.as_mut().unwrap().next.is_some() {
        fast = &mut fast.as_mut().unwrap().next;
        slow = &mut slow.as_mut().unwrap().next;
    }

    let mut slow_clone = slow.clone();
    // head_clone = delete_node(head_clone, &mut slow_clone);
    delete_node(head_clone, &mut slow_clone)
}

fn delete_node(head: Option<Box<ListNode>>, target: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode{
        val: 0,
        next: head,
    }));
    let mut idx = &mut dummy;
    while let Some(node) = idx {
        let next_node = &mut node.next;
        match next_node {
            None => break,
            Some(nxt) => {
                // 判断是不是要被删除的目标节点
                if nxt == target.as_ref().unwrap() {
                    // 直接nxt 
                    node.next = nxt.next.take();
                    break;
                }
            }
        }
        //
        idx = &mut node.next;
    }
    dummy.unwrap().next
}


fn main() {
}
