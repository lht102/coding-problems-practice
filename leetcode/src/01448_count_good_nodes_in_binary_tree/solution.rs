use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, -10000)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cur_max: i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            if r.val >= cur_max {
                Solution::dfs(&r.left, r.val) + Solution::dfs(&r.right, r.val) + 1
            } else {
                Solution::dfs(&r.left, cur_max) + Solution::dfs(&r.right, cur_max)
            }
        } else {
            0
        }
    }
}
