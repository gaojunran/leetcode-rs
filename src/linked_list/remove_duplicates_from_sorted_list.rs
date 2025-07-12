#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub trait ListNodeExt {
    fn take_next(&mut self) -> Option<Box<ListNode>>;

    fn next(&self) -> &Option<Box<ListNode>>;

    fn next_mut(&mut self) -> &mut Option<Box<ListNode>>;

    fn set_next(&mut self, next: Option<Box<ListNode>>);
}

impl ListNodeExt for Option<Box<ListNode>> {
    fn take_next(&mut self) -> Option<Box<ListNode>> {
        self.as_mut().and_then(|node| node.next.take())
    }

    fn next(&self) -> &Option<Box<ListNode>> {
        &self.as_ref().unwrap().next
    }

    fn next_mut(&mut self) -> &mut Option<Box<ListNode>> {
        &mut self.as_mut().unwrap().next
    }

    fn set_next(&mut self, next: Option<Box<ListNode>>) {
        if let Some(node) = self.as_mut() {
            node.next = next;
        }
    }
}

struct Solution;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;

        while let Some(ref mut node) = cur.next
            && let Some(ref next) = node.next
        {
            if node.val == next.val {
                let val = node.val;
                while let Some(ref n) = cur.next {
                    if n.val != val {
                        break;
                    }
                    cur.next = cur.next.as_mut()?.next.take();
                }
            } else {
                cur = cur.next.as_mut()?;
            }
        }

        dummy.next
    }
}
