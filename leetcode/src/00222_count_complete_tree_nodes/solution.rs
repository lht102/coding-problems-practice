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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left_height = Solution::height(&r.left);
            let right_height = Solution::height(&r.right);
            if left_height == right_height {
                (1 << left_height) + Solution::count_nodes(r.right.clone())
            } else {
                (1 << right_height) + Solution::count_nodes(r.left.clone())
            }
        } else {
            0
        }
    }

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            1 + Solution::height(&r.left)
        } else {
            0
        }
    }
}
