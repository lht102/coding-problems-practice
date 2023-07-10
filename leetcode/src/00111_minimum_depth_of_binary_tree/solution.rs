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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            match (&r.left, &r.right) {
                (None, None) => 1,
                (None, Some(_)) => Self::dfs(&r.right) + 1,
                (Some(_), None) => Self::dfs(&r.left) + 1,
                (Some(_), Some(_)) => Self::dfs(&r.left).min(Self::dfs(&r.right)) + 1,
            }
        } else {
            0
        }
    }
}
