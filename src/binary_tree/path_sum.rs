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

struct Solution {}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut queue = VecDeque::new();
        // 初始化时当前路径总和是根节点的值
        if let Some(node) = root.clone() {
            let val = node.borrow().val;
            queue.push_back((node, val));
        }

        while let Some((node, current_sum)) = queue.pop_front() {
            let node = node.borrow();

            // 如果是叶子节点并且 current_sum 等于 target_sum
            if node.left.is_none() && node.right.is_none() && current_sum == target_sum {
                return true;
            }

            // 将左子节点加入队列
            if let Some(left) = node.left.clone() {
                queue.push_back((left.clone(), current_sum + left.borrow().val));
            }

            // 将右子节点加入队列
            if let Some(right) = node.right.clone() {
                queue.push_back((right.clone(), current_sum + right.borrow().val));
            }
        }

        false
    }
}
