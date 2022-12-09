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

struct Solution;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, i32::MAX, i32::MIN)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cur_min: i32, cur_max: i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let new_min = cur_min.min(r.val);
            let new_max = cur_max.max(r.val);
            let left = Solution::dfs(&r.left, new_min, new_max);
            let right = Solution::dfs(&r.right, new_min, new_max);
            left.max(right)
        } else {
            cur_max - cur_min
        }
    }
}
