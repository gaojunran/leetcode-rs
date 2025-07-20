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
struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn inorder(node: Option<Rc<RefCell<TreeNode>>>, count: &mut i32, k: i32) -> Option<i32> {
            let binding = node?;
            let n = binding.borrow();

            // 遍历左子树
            let left = inorder(n.left.clone(), count, k);
            if left.is_some() {
                return left;
            }

            // 处理当前节点
            *count += 1;
            if *count == k {
                return Some(n.val);
            }

            // 遍历右子树
            let right = inorder(n.right.clone(), count, k);
            if right.is_some() {
                return right;
            }
            None
        }

        let mut count = 0;
        inorder(root, &mut count, k).unwrap_or(-1)
    }
}
