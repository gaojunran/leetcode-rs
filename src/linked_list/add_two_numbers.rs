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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut c1 = l1;
        let mut c2 = l2;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while c1.is_some() || c2.is_some() || carry > 0 {
            let v1 = if let Some(node1) = c1 {
                let val = node1.val;
                c1 = node1.next;
                val
            } else {
                0
            };

            let v2 = if let Some(node2) = c2 {
                let val = node2.val;
                c2 = node2.next;
                val
            } else {
                0
            };

            let sum = v1 + v2 + carry;
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
