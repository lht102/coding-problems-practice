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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            if r.left.is_none() && r.right.is_none() {
                return r.val == 1;
            }
            let left = Solution::evaluate_tree(r.left.clone());
            let right = Solution::evaluate_tree(r.right.clone());
            if r.val == 2 {
                left || right
            } else {
                left && right
            }
        } else {
            false
        }
    }
}
