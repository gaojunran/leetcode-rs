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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(ino: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if post.is_empty() {
                return None;
            }
            let root_val = *post.last()?;
            let root_index = ino.iter().position(|&x| x == root_val).unwrap();

            let left = build(&ino[..root_index], &post[..root_index]);
            let right = build(&ino[root_index + 1..], &post[root_index..post.len() - 1]);

            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left,
                right,
            })))
        }

        build(&inorder, &postorder)
    }
}
