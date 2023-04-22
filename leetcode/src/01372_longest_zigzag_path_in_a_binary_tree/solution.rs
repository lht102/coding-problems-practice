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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)[2]
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 3] {
        if let Some(r) = root {
            let r = r.borrow();
            let left_subtree = Self::dfs(&r.left);
            let right_subtree = Self::dfs(&r.right);
            let left = left_subtree[1] + 1;
            let right = right_subtree[0] + 1;
            [
                left,
                right,
                left.max(left)
                    .max(right)
                    .max(left_subtree[2])
                    .max(right_subtree[2]),
            ]
        } else {
            [-1, -1, -1]
        }
    }
}
