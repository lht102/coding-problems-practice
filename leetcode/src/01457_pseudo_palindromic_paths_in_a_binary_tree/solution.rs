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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, 0)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, bitmap: i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let new_bitmap = bitmap ^ (1 << r.val);
            if r.left.is_none() && r.right.is_none() {
                return i32::from(new_bitmap.count_ones() <= 1);
            }
            Solution::dfs(&r.left, new_bitmap) + Solution::dfs(&r.right, new_bitmap)
        } else {
            0
        }
    }
}
