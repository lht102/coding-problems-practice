use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut q = VecDeque::from([root]);
        while let Some(Some(r)) = q.pop_front() {
            let r = r.borrow();
            q.push_back(r.left.clone());
            q.push_back(r.right.clone());
        }
        while let Some(r) = q.pop_front() {
            if r.is_some() {
                return false;
            }
        }
        true
    }
}
