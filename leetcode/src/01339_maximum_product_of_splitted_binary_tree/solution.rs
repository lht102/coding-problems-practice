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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let sum = Solution::dfs_sum(&root);
        let mut res = 0;
        Solution::dfs(&root, sum, &mut res);
        (res % 1_000_000_007) as _
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, total_sum: i64, res: &mut i64) -> i64 {
        if let Some(r) = root {
            let r = r.borrow();
            let cur_sum = r.val as i64
                + Solution::dfs(&r.left, total_sum, res)
                + Solution::dfs(&r.right, total_sum, res);
            *res = (*res).max((total_sum - cur_sum) * cur_sum);
            cur_sum
        } else {
            0
        }
    }

    fn dfs_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
        if let Some(r) = root {
            let r = r.borrow();
            r.val as i64 + Solution::dfs_sum(&r.left) + Solution::dfs_sum(&r.right)
        } else {
            0
        }
    }
}
