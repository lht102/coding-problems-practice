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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::dfs(&root, None, None)
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        min_limit: Option<i32>,
        max_limit: Option<i32>,
    ) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            if let Some(val) = min_limit {
                if r.val <= val {
                    return false;
                }
            }
            if let Some(val) = max_limit {
                if r.val >= val {
                    return false;
                }
            }

            Solution::dfs(&r.left, min_limit, Some(r.val))
                && Solution::dfs(&r.right, Some(r.val), max_limit)
        } else {
            true
        }
    }
}
