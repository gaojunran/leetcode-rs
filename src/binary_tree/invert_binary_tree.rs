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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert(root)
    }

    fn invert(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(node) => {
                let left = Self::invert(node.borrow_mut().left.clone()); // 翻转左子树
                let right = Self::invert(node.borrow_mut().right.clone()); // 翻转右子树
                node.borrow_mut().left = right; // 交换左右儿子
                node.borrow_mut().right = left;
                Some(node)
            }
            None => None,
        }
    }
}
