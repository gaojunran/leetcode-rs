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
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let mut queue = VecDeque::new();
        Self::travel(root, &mut queue);

        while let Some(node) = queue.pop_front() {
            node.borrow_mut().left = None;
            if !queue.is_empty() {
                node.borrow_mut().right = Some(queue.front().unwrap().clone());
            }
        }
    }

    pub fn travel(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        queue: &mut VecDeque<Rc<RefCell<TreeNode>>>,
    ) {
        if root.is_none() {
            return;
        }

        let root = root.as_mut().unwrap();

        queue.push_back(root.clone());

        Self::travel(&mut root.borrow_mut().left, queue);
        Self::travel(&mut root.borrow_mut().right, queue);
    }
}
