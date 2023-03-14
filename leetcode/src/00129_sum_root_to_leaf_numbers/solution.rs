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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cur: i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let new_cur = cur * 10 + r.val;
            if r.left.is_none() && r.right.is_none() {
                return new_cur;
            }

            Self::dfs(&r.left, new_cur) + Self::dfs(&r.right, new_cur)
        } else {
            0
        }
    }
}
