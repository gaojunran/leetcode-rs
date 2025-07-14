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
//     pub fn is_same_tree(
//         p: Option<Rc<RefCell<TreeNode>>>,
//         q: Option<Rc<RefCell<TreeNode>>>,
//     ) -> bool {
//         Solution::is_same_tree_impl(p, q)
//     }
//     pub fn is_same_tree_impl(
//         p: Option<Rc<RefCell<TreeNode>>>,
//         q: Option<Rc<RefCell<TreeNode>>>,
//     ) -> bool {
//         if p.is_none() && q.is_none() {
//             return true;
//         }
//         if p.is_none() || q.is_none() {
//             return false;
//         }
//         let a = p.as_ref().unwrap().borrow();
//         let b = q.as_ref().unwrap().borrow();

//         if a.val != b.val {
//             return false;
//         }
//         Solution::is_same_tree_impl(a.left.clone(), b.left.clone())
//             && Solution::is_same_tree_impl(a.right.clone(), b.right.clone())
//     }
// }

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
