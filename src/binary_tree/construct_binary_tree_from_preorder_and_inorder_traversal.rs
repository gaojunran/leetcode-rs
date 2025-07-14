use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
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

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(pre: &[i32], ino: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if pre.is_empty() {
                return None;
            }

            let root_val = pre[0];
            let left_size = ino.iter().position(|&x| x == root_val).unwrap();

            let left = build(&pre[1..1 + left_size], &ino[..left_size]);
            let right = build(&pre[1 + left_size..], &ino[left_size + 1..]);

            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left,
                right,
            })))
        }

        build(&preorder, &inorder)
    }
}
