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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref r) = root {
            let r = r.borrow();
            let p_val = p.as_ref().unwrap().borrow().val;
            let q_val = q.as_ref().unwrap().borrow().val;
            if p_val < r.val && q_val < r.val {
                return Solution::lowest_common_ancestor(r.left.clone(), p, q);
            }
            if p_val > r.val && q_val > r.val {
                return Solution::lowest_common_ancestor(r.right.clone(), p, q);
            }
        }
        root
    }
}
