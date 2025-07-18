use std::cell::RefCell;
use std::rc::Rc;

// 定义树节点结构
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution {}

// 实现 Solution
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                Self::count_nodes(node.left.clone()) + Self::count_nodes(node.right.clone()) + 1
            }
            None => 0,
        }
    }
}
