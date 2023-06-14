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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MAX;
        Self::dfs(&root, &mut None, &mut res);
        res
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min_diff: &mut i32) {
        if let Some(r) = root {
            let r = r.borrow();
            Self::dfs(&r.left, prev, min_diff);
            if let Some(p) = prev {
                *min_diff = (*min_diff).min(r.val - *p);
            }
            *prev = Some(r.val);
            Self::dfs(&r.right, prev, min_diff);
        }
    }
}
