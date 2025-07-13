// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut len = 0;
        let mut tail = &mut head;
        while let Some(node) = tail {
            len += 1;
            tail = &mut node.next;
        }
        if len == 0 {
            return head;
        }

        let k = k % len;
        if k == 0 {
            return head;
        }

        let mut tail = &mut head;
        for _ in 0..(len - k - 1) {
            tail = &mut tail.as_mut()?.next;
        }
        // take() 直接让 原链表的 尾部为 None
        let mut new_head = tail.as_mut()?.next.take();
        let mut new_tail = &mut new_head;
        // while let Some(node) = new_tail {
        //     new_tail = &mut node.next;
        // }
        // *new_tail = head;
        // new_head
        while new_tail.is_some() && new_tail.as_mut()?.next.is_some() {
            new_tail = &mut new_tail.as_mut()?.next;
        }
        new_tail.as_mut()?.next = head;
        new_head
    }
}
