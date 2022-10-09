use std::cell::RefCell;
use std::collections::HashSet;
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        Solution::dfs(&root, k, &mut HashSet::new())
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, visited: &mut HashSet<i32>) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            if visited.contains(&r.val) {
                return true;
            }
            visited.insert(k - r.val);
            Solution::dfs(&r.left, k, visited) || Solution::dfs(&r.right, k, visited)
        } else {
            false
        }
    }
}
