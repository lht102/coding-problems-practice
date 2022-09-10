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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut st = vec![];
        Solution::push_left(&root, &mut st);
        while let Some(r) = st.pop() {
            let r = r.borrow();
            res.push(r.val);
            Solution::push_left(&r.right, &mut st);
        }
        res
    }

    fn push_left(root: &Option<Rc<RefCell<TreeNode>>>, st: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let mut p = root.clone();
        while let Some(r) = p {
            st.push(r.clone());
            p = r.borrow().left.clone();
        }
    }
}
