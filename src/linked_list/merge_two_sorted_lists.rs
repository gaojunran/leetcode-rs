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
    #[inline]
    fn boxed(val: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(val)))
    }

    #[inline]
    fn take(node: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        if let Some(n) = node {
            (n.val, n.next)
        } else {
            (0, None)
        }
    }
}
struct Solution;
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let l1_val = list1.as_ref().unwrap().val;
            let l2_val = list2.as_ref().unwrap().val;

            if l1_val < l2_val {
                let next = list1.as_mut().unwrap().next.take();
                tail.next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                tail.next = list2;
                list2 = next;
            }

            tail = tail.next.as_mut().unwrap();
        }

        // 连接剩余部分
        tail.next = if list1.is_some() { list1 } else { list2 };

        dummy.next
    }
}

// impl ListNode {
//     #[inline]
//     fn boxed(val: i32) -> Option<Box<ListNode>> {
//         Some(Box::new(ListNode::new(val)))
//     }

//     /// 拆出当前节点作为一个整体 + 剩余 next
//     fn split_node(node: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
//         if let Some(mut n) = node {
//             let next = n.next.take();
//             (Some(n), next)
//         } else {
//             (None, None)
//         }
//     }
// }

// impl Solution {
//     pub fn merge_two_lists(
//         mut list1: Option<Box<ListNode>>,
//         mut list2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         let mut dummy = ListNode::boxed(0);
//         let mut tail = &mut dummy;

//         while list1.is_some() && list2.is_some() {
//             let (picked, next) = if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
//                 ListNode::split_node(list1)
//             } else {
//                 ListNode::split_node(list2)
//             };

//             if list1.as_ref().map_or(false, |n| n.val < list2.as_ref().unwrap().val) {
//                 list1 = next;
//             } else {
//                 list2 = next;
//             }

//             tail.as_mut().unwrap().next = picked;
//             tail = &mut tail.as_mut().unwrap().next;
//         }

//         tail.as_mut().unwrap().next = if list1.is_some() { list1 } else { list2 };
//         dummy.unwrap().next
//     }
// }
