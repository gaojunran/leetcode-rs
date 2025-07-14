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
// impl Solution {
//     pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         match root {
//             Some(node) => {
//                 // 左子树最大深度
//                 let left = Self::max_depth(node.borrow().left.clone());
//                 // 右子树最大深度
//                 let right = Self::max_depth(node.borrow().right.clone());
//                 // 比较左右子树深度，取较大值加上根节点
//                 1 + left.max(right)
//             }
//             _ => 0,
//         }
//     }
// }

pub trait TreeNodeExt {
    fn left(&self) -> Option<Rc<RefCell<TreeNode>>>;
    fn right(&self) -> Option<Rc<RefCell<TreeNode>>>;
}

impl TreeNodeExt for Rc<RefCell<TreeNode>> {
    fn left(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.borrow().left.clone()
    }

    fn right(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.borrow().right.clone()
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

        let mut depth = 0;
        let mut deque = VecDeque::new();

        if let Some(node) = root {
            deque.push_back(node);
        }

        while !deque.is_empty() {
            depth += 1;
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    let node = node.borrow();
                    if let Some(ref left) = node.left {
                        deque.push_back(left.clone());
                    }
                    if let Some(ref right) = node.right {
                        deque.push_back(right.clone());
                    }
                }
            }
        }

        depth
    }
}
