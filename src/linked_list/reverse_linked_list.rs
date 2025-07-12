// 链表节点定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // 构造函数
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub trait ListNodeExt {
    fn get_next(&mut self) -> Option<Box<ListNode>>;
    fn set_next(&mut self, next: Option<Box<ListNode>>);
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
}

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // dummy 节点简化边界操作
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;

        // 定位到 left 前一个节点
        for _ in 1..left {
            prev = prev.next.as_mut().unwrap();
        }

        // 开始反转
        let mut curr = prev.next.take(); // curr 是反转区域的头
        let mut next;

        for _ in left..right {
            // 经典头插法反转
            next = curr.get_next(); // 拿到 curr.next
            curr.set_next(next.get_next()); // curr.next = next.next
            next.set_next(prev.next.take()); // next.next = prev.next
            prev.next = next; // prev.next = next
        }

        // 将反转好的 curr 接上
        let mut tail = prev.next.as_mut().unwrap();
        while let Some(ref mut node) = tail.next {
            tail = node;
        }
        tail.next = curr;

        dummy.next
    }
}
