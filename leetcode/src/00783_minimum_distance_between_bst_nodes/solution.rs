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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MAX;
        Self::dfs(&root, &mut None, &mut res);
        res
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, res: &mut i32) {
        if let Some(r) = root {
            let r = r.borrow();
            if r.left.is_some() {
                Self::dfs(&r.left, prev, res);
            }
            if let Some(p) = prev {
                *res = (*res).min(r.val - *p);
            }
            *prev = Some(r.val);
            if r.right.is_some() {
                Self::dfs(&r.right, prev, res);
            }
        }
    }
}
