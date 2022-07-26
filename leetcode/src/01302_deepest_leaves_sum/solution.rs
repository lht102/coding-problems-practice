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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut q = VecDeque::new();
        if let Some(v) = &root {
            q.push_back(v.clone());
        }
        while !q.is_empty() {
            let mut csum = 0;
            for _ in 0..q.len() {
                if let Some(front) = q.pop_front() {
                    let front = front.borrow();
                    csum += front.val;
                    if let Some(left) = &front.left {
                        q.push_back(left.clone());
                    }
                    if let Some(right) = &front.right {
                        q.push_back(right.clone());
                    }
                }
            }
            res = csum;
        }
        res
    }
}
