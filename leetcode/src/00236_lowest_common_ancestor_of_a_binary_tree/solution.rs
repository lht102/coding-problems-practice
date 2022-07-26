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
        Solution::dfs(root, p.as_ref(), q.as_ref())
    }

    pub fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<&Rc<RefCell<TreeNode>>>,
        q: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref r) = root {
            if Some(r) == p || Some(r) == q {
                return root;
            }
            let r = r.borrow();
            let left = Solution::dfs(r.left.clone(), p, q);
            let right = Solution::dfs(r.right.clone(), p, q);
            if left.is_none() {
                return right;
            }
            if right.is_none() {
                return left;
            }
        }
        root
    }
}
