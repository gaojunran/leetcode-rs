struct Solution;

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

// 双指针同样违背了所有权规则（两个可变借用）
// impl Solution {
//     pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
//         let mut tail = &mut head;
//         // 遍历一圈，拿到最后的 None 节点
//         while let Some(node) = tail {
//             tail = &mut node.next;
//         }
//         // 再遍历一圈，把所有大于 x 的节点放到 tail 后
//         // TODO
//         head
//     }
// }
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less_dummy = Box::new(ListNode { val: 0, next: None });
        let mut greater_dummy = Box::new(ListNode { val: 0, next: None });

        let (mut less_tail, mut greater_tail) = (&mut less_dummy, &mut greater_dummy);
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take(); // 拿出 node 的下一个节点（断开原链）
            if node.val < x {
                less_tail.next = Some(node);
                less_tail = less_tail.next.as_mut()?;
            } else {
                greater_tail.next = Some(node);
                greater_tail = greater_tail.next.as_mut()?;
            }
        }

        // 拼接两个链表
        less_tail.next = greater_dummy.next;
        less_dummy.next
    }
}
