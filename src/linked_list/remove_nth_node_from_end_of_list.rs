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
    /// 拿走当前节点的 next，原位置设为 None
    fn take_next(&mut self) -> Option<Box<ListNode>>;

    /// 获取当前节点 next 的不可变引用
    fn get_next(&self) -> &Option<Box<ListNode>>;

    /// 获取当前节点 next 的可变引用
    fn get_next_mut(&mut self) -> &mut Option<Box<ListNode>>;

    /// 设置当前节点的 next
    fn set_next(&mut self, next: Option<Box<ListNode>>);

    // fn advance_ref(&self, n: usize) -> &Option<Box<ListNode>>;
    // /// 向后推进 n 步，返回推进后的可变引用
    // fn advance_mut(&mut self, n: usize) -> &mut Option<Box<ListNode>>;
    // fn advance(&self, n: usize) -> Option<Box<ListNode>>;
}

impl ListNodeExt for Option<Box<ListNode>> {
    fn take_next(&mut self) -> Option<Box<ListNode>> {
        self.as_mut().and_then(|node| node.next.take())
    }

    fn get_next(&self) -> &Option<Box<ListNode>> {
        &self.as_ref().unwrap().next
    }

    fn get_next_mut(&mut self) -> &mut Option<Box<ListNode>> {
        &mut self.as_mut().unwrap().next
    }

    fn set_next(&mut self, next: Option<Box<ListNode>>) {
        if let Some(node) = self.as_mut() {
            node.next = next;
        }
    }

    // fn advance_mut(&mut self, n: usize) -> &mut Option<Box<ListNode>> {
    //     let mut ptr = self;
    //     for _ in 0..n {
    //         ptr = &mut ptr.as_mut()?.next;
    //     }
    //     ptr
    // }

    // fn advance_ref(&self, n: usize) -> &Option<Box<ListNode>> {
    //     let mut ptr = self;
    //     for _ in 0..n {
    //         ptr = &ptr.as_ref()?.next;
    //     }
    //     ptr
    // }
}

struct Solution;

// impl Solution {  // wrong
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));

//         let mut fast = dummy.advance_mut(n as usize + 1);
//         let mut slow = &mut dummy;

//         while fast.get_next().is_some() {
//             fast = fast.advance_mut(1);
//             slow = slow.advance_mut(1);
//         }

//         // 删除 slow.next
//         let next = slow.take_next().and_then(|n| n.next);
//         slow.set_next(next);

//         dummy?.next
//     }
// }
impl Solution {}

// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let mut head = head;
//         let mut slow = &mut head;
//         let mut fast = &slow.clone();
//         for _ in 0..n + 1 {
//             if fast.is_none() {
//                 return head.as_mut()?.next.take();
//             }
//             fast = &fast.as_ref()?.next;
//         }
//         while fast.is_some() {
//             fast = &fast.as_ref()?.next;
//             slow = &mut slow.as_mut()?.next;
//         }
//         slow.as_mut()?.next = slow.as_mut()?.next.as_mut()?.next.take();
//         head
//     }
// }

// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));

//         // ✅ 在可变借用 dummy 之前先 clone 出 fast 用的链表
//         let mut fast = &mut dummy.as_mut()?.next.clone();

//         let mut slow = &mut dummy;

//         for _ in 0..n {
//             fast = fast.get_next_mut();
//         }

//         while fast.is_some() {
//             fast = fast.get_next_mut();
//             slow = slow.get_next_mut();
//         }

//         let next = slow.get_next_mut().get_next_mut().clone();
//         slow.set_next(next);

//         dummy.take_next()
//     }
// }

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        let mut result = None;
        let mut tail = &mut result;

        for _ in 0..n - 1 {
            fast = &fast.as_ref()?.next;
        }
        while fast.as_ref()?.next.is_some() {
            *tail = Some(Box::new(ListNode::new(slow.as_ref()?.val)));
            tail = &mut tail.as_mut()?.next;
            fast = &fast.as_ref()?.next;
            slow = &slow.as_ref()?.next;
        }
        *tail = slow.as_ref()?.next.to_owned();
        result
    }
}
