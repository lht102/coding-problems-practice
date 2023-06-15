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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        if let Some(r) = root {
            q.push_back(r);
        }
        let mut res = 1;
        let mut max_sum = i32::MIN;
        let mut level = 1;
        while !q.is_empty() {
            let mut level_sum = 0;
            for _ in 0..q.len() {
                if let Some(r) = q.pop_front() {
                    let r = r.borrow();
                    level_sum += r.val;
                    if let Some(left) = &r.left {
                        q.push_back(left.clone());
                    }
                    if let Some(right) = &r.right {
                        q.push_back(right.clone());
                    }
                }
            }
            if max_sum < level_sum {
                max_sum = level_sum;
                res = level;
            }
            level += 1;
        }
        res
    }
}
