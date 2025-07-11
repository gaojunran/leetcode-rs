use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

struct ListNode{
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode{
    pub fn new(val: i32) -> ListNode {
        ListNode { 
            val, 
            next: None,
        }
    }
}

pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut set = HashSet::new();

    let mut h = match head {
        Some(ref node) => Rc::clone(node),
        _ => return false,
    };

    loop {
        if set.contains(&Rc::as_ptr(&h)) { return true; }
        set.insert(Rc::as_ptr(&h));
        let next = match h.borrow().next {
            Some(ref node) => Rc::clone(node),
            _ => break,
        };
        h = next;
    }

    false
}
