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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inorder(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut i32, min_diff: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                inorder(n.left.clone(), prev, min_diff);

                if *prev != i32::MAX {
                    *min_diff = (*min_diff).min((n.val - *prev).abs());
                }
                *prev = n.val;

                inorder(n.right.clone(), prev, min_diff);
            }
        }

        let mut prev = i32::MAX;
        let mut min_diff = i32::MAX;
        inorder(root, &mut prev, &mut min_diff);
        min_diff
    }
}
