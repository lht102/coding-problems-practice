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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        *root = Solution::dfs(root.take(), None);
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        prev: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref r) = root {
            let mut r = r.borrow_mut();
            let right = Solution::dfs(r.right.take(), prev);
            r.right = Solution::dfs(r.left.take(), right);
        } else {
            return prev;
        }
        root
    }
}
