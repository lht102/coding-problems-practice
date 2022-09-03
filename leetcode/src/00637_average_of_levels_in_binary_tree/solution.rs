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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut q = VecDeque::new();
        if let Some(r) = &root {
            q.push_back(r.clone());
        }
        let mut res = Vec::new();
        while !q.is_empty() {
            let mut level_sum = 0.0;
            let level_cnt = q.len();
            for _ in 0..q.len() {
                if let Some(front) = q.pop_front() {
                    let front = front.borrow();
                    level_sum += front.val as f64;
                    if let Some(left) = &front.left {
                        q.push_back(left.clone());
                    }
                    if let Some(right) = &front.right {
                        q.push_back(right.clone());
                    }
                }
            }
            res.push(((level_sum / level_cnt as f64) * 100000.0).round() / 100000.0);
        }
        res
    }
}
