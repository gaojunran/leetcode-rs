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
    /// 拿出当前节点的 next（会把 next 设为 None）
    fn get_next(&mut self) -> Option<Box<ListNode>>;
    /// 设置当前节点的 next
    fn set_next(&mut self, next: Option<Box<ListNode>>);
    /// 向后推进 n 步，返回推进后的可变引用
    fn advance_mut(&mut self, n: usize) -> &mut Option<Box<ListNode>>;
}

impl ListNodeExt for Option<Box<ListNode>> {
    fn get_next(&mut self) -> Option<Box<ListNode>> {
        self.as_mut().and_then(|node| node.next.take())
    }

    fn set_next(&mut self, next: Option<Box<ListNode>>) {
        if let Some(node) = self.as_mut() {
            node.next = next;
        }
    }

    fn advance_mut(&mut self, n: usize) -> &mut Option<Box<ListNode>> {
        let mut ptr = self;
        for _ in 0..n {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        ptr
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));

        // prev 指向 left 的前一个节点
        let prev = dummy.advance_mut((left - 1) as usize);

        // curr 是反转段第一个节点
        let mut curr = prev.get_next();

        for _ in left..right {
            let mut next = curr.get_next();

            // curr.next = next.next
            curr.set_next(next.get_next());

            // next.next = prev.next
            next.set_next(prev.get_next());

            // prev.next = next
            prev.set_next(next);
        }

        // 找反转后链表尾部 tail，把 curr 接上
        let mut tail = prev.advance_mut(1);
        while tail.as_mut().unwrap().next.is_some() {
            // 这里也可以写成：tail = tail.advance_mut(1);
            tail = tail.advance_mut(1);
        }
        tail.set_next(curr);

        dummy.unwrap().next
    }
}
