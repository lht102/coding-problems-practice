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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MIN;
        Solution::dfs(&root, &mut res);
        res
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left = Solution::dfs(&r.left, res).max(0);
            let right = Solution::dfs(&r.right, res).max(0);
            *res = (*res).max(left + right + r.val);
            r.val + left.max(right)
        } else {
            0
        }
    }
}
