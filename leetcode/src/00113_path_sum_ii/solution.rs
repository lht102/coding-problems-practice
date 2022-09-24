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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cur = Vec::new();
        Solution::dfs(&root, target_sum, &mut cur, &mut res);
        res
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            cur.push(r.val);
            let remaining = target_sum - r.val;
            if r.left.is_none() && r.right.is_none() && remaining == 0 {
                res.push(cur.clone());
            } else {
                Solution::dfs(&r.left, remaining, cur, res);
                Solution::dfs(&r.right, remaining, cur, res);
            }
            cur.pop();
        }
    }
}
