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
struct Solution {}
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, prev: i32) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let sum = prev * 10 + node.val;

                    // 如果是叶子节点
                    if node.left.is_none() && node.right.is_none() {
                        sum
                    } else {
                        dfs(node.left.clone(), sum) + dfs(node.right.clone(), sum)
                    }
                }
                None => 0,
            }
        }

        dfs(root, 0)
    }
}
