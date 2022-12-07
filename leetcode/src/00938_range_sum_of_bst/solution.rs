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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut res = 0;
        Solution::dfs(&root, low, high, &mut res);
        res
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, res: &mut i32) {
        if let Some(r) = root {
            let r = r.borrow();
            if r.val >= low && r.val <= high {
                *res += r.val;
            }
            if r.val > low {
                Solution::dfs(&r.left, low, high, res);
            }
            if r.val < high {
                Solution::dfs(&r.right, low, high, res);
            }
        }
    }
}
