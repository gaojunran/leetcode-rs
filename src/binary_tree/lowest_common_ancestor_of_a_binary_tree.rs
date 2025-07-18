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

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        // 获取目标节点值
        let p_val = p.as_ref()?.borrow().val;
        let q_val = q.as_ref()?.borrow().val;

        // 记录两条路径
        let mut path_p = vec![];
        let mut path_q = vec![];

        // 如果找不到路径，直接返回 None
        if !Self::find_path(&root, p_val, &mut path_p)
            || !Self::find_path(&root, q_val, &mut path_q)
        {
            return None;
        }

        // 找最后一个公共节点
        let mut i = 0;
        while i < path_p.len() && i < path_q.len() {
            if Rc::ptr_eq(&path_p[i], &path_q[i]) {
                i += 1;
            } else {
                break;
            }
        }

        Some(path_p[i - 1].clone())
    }

    fn find_path(node: &Node, target: i32, path: &mut Vec<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = node {
            path.push(n.clone());
            let val = n.borrow().val;

            if val == target {
                return true;
            }

            if Self::find_path(&n.borrow().left, target, path)
                || Self::find_path(&n.borrow().right, target, path)
            {
                return true;
            }

            path.pop(); // 回溯
        }

        false
    }
}
