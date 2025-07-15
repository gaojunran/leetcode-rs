struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                Self::is_mirror(node.left.clone(), node.right.clone())
            }
            None => true,
        }
    }

    fn is_mirror(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val
                    && Self::is_mirror(a.left.clone(), b.right.clone())
                    && Self::is_mirror(a.right.clone(), b.left.clone())
            }
            _ => false,
        }
    }
}

// use std::cell::RefCell;
// use std::rc::Rc;
// use std::collections::VecDeque;

// impl Solution {
//     pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         let mut queue = VecDeque::new();

//         if let Some(node) = root {
//             let node = node.borrow();
//             queue.push_back((node.left.clone(), node.right.clone()));
//         }

//         while let Some((t1, t2)) = queue.pop_front() {
//             match (t1, t2) {
//                 (None, None) => continue,
//                 (Some(a), Some(b)) => {
//                     let a = a.borrow();
//                     let b = b.borrow();
//                     if a.val != b.val {
//                         return false;
//                     }
//                     // 保持对称结构地入队
//                     queue.push_back((a.left.clone(), b.right.clone()));
//                     queue.push_back((a.right.clone(), b.left.clone()));
//                 }
//                 _ => return false, // 一个有值一个没值
//             }
//         }

//         true
//     }
// }
