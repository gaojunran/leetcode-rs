use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let val = n.borrow().val as i64;
                    if val <= min || val >= max {
                        return false;
                    }
                    helper(&n.borrow().left, min, val) && helper(&n.borrow().right, val, max)
                }
            }
        }

        helper(&root, i64::MIN, i64::MAX)
    }
}
