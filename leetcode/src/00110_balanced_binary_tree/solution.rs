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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::height(&root) != -1
    }

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left_d = Solution::height(&r.left);
            let right_d = Solution::height(&r.right);

            if left_d == -1 || right_d == -1 || (left_d - right_d).abs() > 1 {
                return -1;
            }
            left_d.max(right_d) + 1
        } else {
            0
        }
    }
}
