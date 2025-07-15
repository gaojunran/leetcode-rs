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
use std::collections::VecDeque;
use std::rc::Rc;

// 假设 TreeNode 已经定义好

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut sum = 0.0;

            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    sum += node.val as f64;
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }

            result.push(sum / level_size as f64);
        }

        result
    }
}
