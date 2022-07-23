use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

// Definition for a binary tree node.
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut q = VecDeque::new();
        if let Some(r) = root {
            q.push_back(r);
        }
        while !q.is_empty() {
            let mut cur = Vec::with_capacity(q.len());
            for _ in 0..q.len() {
                if let Some(front) = q.pop_front() {
                    let front = front.borrow();
                    cur.push(front.val);
                    if let Some(left) = &front.left {
                        q.push_back(left.clone());
                    }
                    if let Some(right) = &front.right {
                        q.push_back(right.clone());
                    }
                }
            }
            res.push(cur)
        }
        res
    }
}
