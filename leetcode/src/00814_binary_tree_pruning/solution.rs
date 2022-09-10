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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(r_rc) => {
                let mut r_ref = r_rc.borrow_mut();
                match (
                    r_ref.val,
                    Self::prune_tree(r_ref.left.take()),
                    Self::prune_tree(r_ref.right.take()),
                ) {
                    (0, None, None) => None,
                    (_, left, right) => {
                        r_ref.left = left;
                        r_ref.right = right;
                        drop(r_ref);
                        Some(r_rc)
                    }
                }
            }
        }
    }
}
